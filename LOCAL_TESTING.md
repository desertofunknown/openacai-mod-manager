# Local Testing

Use the refresh script to keep the GitHub prebuilt files and local test executable in sync with the latest Tauri build:

```powershell
.\scripts\refresh-prebuilt.ps1 -Build
```

The script copies the portable executable to:

```text
local-test\OpenACAI-Mod-Manager-0.1.0-x64-portable.exe
```

`local-test/` is ignored by Git so test-only binaries stay local.
