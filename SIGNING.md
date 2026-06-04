# Code Signing

The current public Windows builds are unsigned. Windows SmartScreen and some browsers can warn on unsigned executables, especially from a new GitHub project with little reputation.

Checksums help users verify that a download matches the file published by this repository, but checksums do not replace code signing.

## Recommended Signing Path

1. Obtain an OV or EV code-signing certificate for OpenACAI/Alex Cooper.
2. Install the certificate in the Windows certificate store on the release build machine.
3. Configure Tauri's Windows signing options in `src-tauri/tauri.conf.json` or inject them during release automation:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": "CERTIFICATE_SHA1_THUMBPRINT",
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.digicert.com"
      }
    }
  }
}
```

Do not commit private keys, certificate passwords, or token credentials.

## Current Mitigation

- Publish SHA256 checksums in `prebuilt/SHA256SUMS.txt`.
- Provide a clearly branded portable executable and NSIS setup installer.
- Do not publish the MSI while its launch issue is unresolved.
