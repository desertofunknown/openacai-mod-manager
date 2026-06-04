# Prebuilt Installers

This folder contains the current Windows desktop builds of OpenACAI Mod Manager.

- `OpenACAI-Mod-Manager-0.1.1-x64-portable.exe`: direct portable app executable.
- `OpenACAI-Mod-Manager-0.1.1-x64-setup.exe`: NSIS setup executable.
- `OpenACAI-Mod-Manager.ico`: OpenACAI Windows icon used by the app and installer.
- `SHA256SUMS.txt`: checksums for the portable executable, setup installer, and icon.

MSI builds are intentionally not published while the current MSI launch issue is investigated.

These builds are unsigned, so Windows SmartScreen and some browsers may warn on first download. Verify checksums before running. A trusted code-signing certificate is required to materially reduce those warnings.

Dependency/source links:

- OpenACAI Loader package: `https://github.com/desertofunknown/openacai-loader`
- BepInEx: `https://github.com/BepInEx/BepInEx`
- RedLoader/SonsSdk rewrite branch: `https://github.com/ToniMacaroni/RedLoader/tree/rewrite`
- Original RedManager upstream: `https://github.com/ToniMacaroni/RedManager`

The private OpenACAI anti-cheat/admin mod is not included in this manager or its installers.
