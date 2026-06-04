# OpenACAI Mod Manager

[![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Built With: Tauri](https://img.shields.io/badge/built%20with-Tauri-5aa8ff.svg)](https://tauri.app/)
[![Game: Sons Of The Forest](https://img.shields.io/badge/game-Sons%20Of%20The%20Forest-38d68d.svg)](https://store.steampowered.com/app/1326470/Sons_Of_The_Forest/)

Developer: Alex Cooper

OpenACAI Mod Manager is a Tauri/Svelte manager for installing and maintaining the OpenACAI Loader package for Sons of the Forest.

It is based on Toni Macaroni's RedManager, but the main loader flow has been changed for OpenACAI Loader:

- Select and install a local `OpenACAILoader.zip`.
- Detect the OpenACAI Loader bridge through `BepInEx\plugins\OpenACAILoader` and `BepInEx\plugins\RedLoaderBepInExCompat`.
- Keep old RedLoader/MelonLoader cleanup affordances so users can avoid competing loader bootstraps.

This public manager does not include the private OpenACAI anti-cheat/admin mod.

Brand asset provenance is documented in `BRANDING.md`.
Upstream/fork lineage is documented in `UPSTREAMS.md`.

## Screenshots

![OpenACAI Mod Manager main tab](docs/screenshots/openacai-mod-manager-main.png)

## Prebuilt Downloads

The current Windows builds are kept in `prebuilt/` for direct GitHub downloads:

- [Windows portable executable](prebuilt/OpenACAI-Mod-Manager-0.1.1-x64-portable.exe)
- [Windows setup executable](prebuilt/OpenACAI-Mod-Manager-0.1.1-x64-setup.exe)
- [OpenACAI icon](prebuilt/OpenACAI-Mod-Manager.ico)
- [SHA256 checksums](prebuilt/SHA256SUMS.txt)

These builds are unsigned, so Windows SmartScreen and some browsers may warn on first download. Verify the SHA256 checksum before running. Code-signing guidance is documented in `SIGNING.md`.

GitHub release assets should mirror this folder when tagged releases are created. MSI builds are intentionally not published while the current MSI launch issue is investigated.

Local test-copy workflow is documented in `LOCAL_TESTING.md`.

## Development

```powershell
npm ci
npm run check
npm run build
```

For the desktop shell:

```powershell
npm run tauri dev
```

To refresh prebuilt files and the ignored local portable test executable:

```powershell
.\scripts\refresh-prebuilt.ps1 -Build
```

## Licensing

OpenACAI-authored manager changes are licensed under `GPL-3.0-or-later`.

The original RedManager project is licensed under `Apache-2.0`; its license text is preserved in `LICENSES/Apache-2.0.txt`, and attribution is preserved in `THIRD_PARTY_NOTICES.md`.

The OpenACAI Loader packages managed by this app are built on BepInEx and RedLoader/SonsSdk components. Their LGPL notices are preserved by the loader package and called out in this manager's third-party notices.
