// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::fs;
use zip::read::ZipArchive;

use std::io::{BufReader, BufRead};
use winreg::{RegKey, enums::*};
use regex::Regex;
use serde::Serialize;

use std::{error::Error, path::Path, path::PathBuf, ptr::null_mut};
use windows::{
    core,
    Win32::Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO},
};

const SOTF_APP_ID: &str = "1326470";

#[derive(Serialize)]
struct LoaderZipInspection {
    is_valid: bool,
    has_manifest: bool,
    has_bridge: bool,
    has_bepinex_core: bool,
    has_doorstop: bool,
    has_private_admin_tool: bool,
    entries: usize,
    errors: Vec<String>,
}

#[tauri::command]
fn is_dotnet6_installed() -> Result<bool, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey_with_flags(
        r"SOFTWARE\WOW6432Node\dotnet\Setup\InstalledVersions\x64\sharedfx\Microsoft.NETCore.App", 
        KEY_READ
    ).map_err(|op| op.to_string())?;

    let contains_dotnet_6 = key
        .enum_values()
        .filter_map(Result::ok)
        .any(|(name, _)| name.starts_with("6."));

    Ok(contains_dotnet_6)
}

#[tauri::command]
fn get_file_version(path: String) -> Result<String, String> {
    let desc = get_file_description(path).map_err(|e| e.to_string())?;
    Ok(desc)
}

fn get_file_description(path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let size = unsafe { GetFileVersionInfoSizeW(path.as_ref().as_os_str(), null_mut()) };
    if size == 0 {
        return Err(core::Error::from_win32().into());
    }

    let mut buffer = vec![0u8; size as usize];
    unsafe {
        GetFileVersionInfoW(
            path.as_ref().as_os_str(),
            0,
            size,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
        )
    }
    .ok()?;

    let mut ptr = null_mut();
    let mut len = 0;
    let success = unsafe {
        VerQueryValueW(
            buffer.as_ptr() as *const std::ffi::c_void,
            "\\",
            &mut ptr,
            &mut len,
        )
    }.as_bool();

    if !success {
        return Err("Failed to query file description".into());
    }

    let info = ptr as *const VS_FIXEDFILEINFO;
    unsafe{
        if (*info).dwSignature != 0xfeef04bd {
            return Err("Invalid fixed file info signature".into());
        }

        let description = *info;
        
        Ok(format!("{}.{}.{}", 
            description.dwFileVersionMS >> 16,
            description.dwFileVersionMS & 0xffff,
            description.dwFileVersionLS >> 16,
        ))
    }

}

#[tauri::command]
async fn get_steam_path() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_install: PathBuf = hklm.open_subkey_with_flags(r"SOFTWARE\WOW6432Node\Valve\Steam", KEY_READ)
    .and_then(|key| key.get_value::<String, _>("InstallPath"))
    .or_else(|_| {
        hklm.open_subkey_with_flags(r"SOFTWARE\Valve\Steam", KEY_READ)
            .and_then(|key| key.get_value::<String, _>("InstallPath"))
    }).ok()?
    .into();

    let vdf = steam_install.join(r"steamapps\libraryfolders.vdf");
    if !vdf.exists() {
        return None;
    }

    let re = Regex::new(r#"\s"(?:\d|path)"\s+"(.+)""#).unwrap();
    let mut steam_paths = vec![steam_install.join(r"steamapps")];

    let file = File::open(vdf).ok()?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(text) = line {
            if let Some(cap) = re.captures(&text) {
                steam_paths.push(PathBuf::from(cap[1].replace(r"\\", r"\")).join(r"steamapps"));
            }
        }
    }

    let install_re = Regex::new(r#"\s"installdir"\s+"(.+)""#).unwrap();
    for path in steam_paths {
        let manifest = path.join(format!(r"appmanifest_{}.acf", SOTF_APP_ID));
        if manifest.exists() {
            let file = File::open(manifest).ok()?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(text) = line {
                    if let Some(cap) = install_re.captures(&text) {
                        let file_path = path.join("common").join(&cap[1]).join("SonsOfTheForest.exe");
                        if file_path.exists() {
                            return file_path.to_str().map(|s| s.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}


fn unzip_file(source: &str, destination: &str) -> Result<(), String> {
    let reader = File::open(source).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(reader).map_err(|e| e.to_string())?;
    let destination_root = fs::canonicalize(destination).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let enclosed_name = file.enclosed_name()
            .ok_or_else(|| format!("Unsafe path in zip entry: {}", file.name()))?
            .to_owned();
        let outpath = destination_root.join(enclosed_name);

        if !outpath.starts_with(&destination_root) {
            return Err(format!("Zip entry escapes destination: {}", file.name()));
        }

        if file.is_dir() {
            if !outpath.exists() {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            }
            continue; // Skip to the next iteration since it's a directory.
        }

        if let Some(parent_dir) = outpath.parent() {
            fs::create_dir_all(parent_dir).map_err(|e| e.to_string())?;
        }

        if outpath.exists() {
            if outpath.is_file() {
                fs::remove_file(&outpath).map_err(|e| e.to_string())?;
            } else {
                return Err(format!("Refusing to overwrite non-file path: {}", outpath.display()));
            }
        }
        
        let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
        std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn normalize_zip_entry(name: &str) -> String {
    name.replace('\\', "/").trim_start_matches("./").to_ascii_lowercase()
}

#[tauri::command]
fn inspect_openacai_loader_zip(source: String) -> Result<LoaderZipInspection, String> {
    let reader = File::open(source).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(reader).map_err(|e| e.to_string())?;
    let mut inspection = LoaderZipInspection {
        is_valid: false,
        has_manifest: false,
        has_bridge: false,
        has_bepinex_core: false,
        has_doorstop: false,
        has_private_admin_tool: false,
        entries: archive.len(),
        errors: Vec::new(),
    };

    for i in 0..archive.len() {
        let file = archive.by_index(i).map_err(|e| e.to_string())?;
        if file.enclosed_name().is_none() {
            inspection.errors.push(format!("Unsafe path in zip entry: {}", file.name()));
            continue;
        }

        let entry = normalize_zip_entry(file.name());
        inspection.has_manifest |= entry == "bepinex/plugins/openacailoader/openacai-loader.manifest.json";
        inspection.has_bridge |= entry == "bepinex/plugins/redloaderbepinexcompat/redloaderbepinexcompat.dll";
        inspection.has_bepinex_core |= entry == "bepinex/core/bepinex.core.dll";
        inspection.has_doorstop |= entry == "winhttp.dll" || entry == "doorstop_config.ini";
        inspection.has_private_admin_tool |= entry.contains("openacaiadmintool")
            || entry.contains("anti-cheat")
            || entry.contains("anticheat");
    }

    if !inspection.has_manifest {
        inspection.errors.push("Missing BepInEx/plugins/OpenACAILoader/openacai-loader.manifest.json".to_string());
    }

    if !inspection.has_bridge {
        inspection.errors.push("Missing BepInEx/plugins/RedLoaderBepInExCompat/RedLoaderBepInExCompat.dll".to_string());
    }

    if !inspection.has_bepinex_core {
        inspection.errors.push("Missing BepInEx/core/BepInEx.Core.dll".to_string());
    }

    if !inspection.has_doorstop {
        inspection.errors.push("Missing Doorstop bootstrap files (winhttp.dll or doorstop_config.ini).".to_string());
    }

    if inspection.has_private_admin_tool {
        inspection.errors.push("Package appears to contain private OpenACAI admin/anti-cheat files; public loader installs must not include them.".to_string());
    }

    inspection.is_valid = inspection.errors.is_empty();
    Ok(inspection)
}

#[tauri::command]
fn unzip_handler(source: String, destination: String) -> Result<(), String> {
    unzip_file(&source, &destination)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![unzip_handler, inspect_openacai_loader_zip, get_steam_path, is_dotnet6_installed, get_file_version])
        .plugin(tauri_plugin_upload::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
