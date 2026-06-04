# OpenACAI Mod Manager

Developer: Alex Cooper

OpenACAI Mod Manager is a Tauri/Svelte manager for installing and maintaining the OpenACAI Loader package for Sons of the Forest.

It is based on Toni Macaroni's RedManager, but the main loader flow has been changed for OpenACAI Loader:

- Select and install a local `OpenACAILoader.zip`.
- Detect the OpenACAI Loader bridge through `BepInEx\plugins\OpenACAILoader` and `BepInEx\plugins\RedLoaderBepInExCompat`.
- Keep old RedLoader/MelonLoader cleanup affordances so users can avoid competing loader bootstraps.

This public manager does not include the private OpenACAI anti-cheat/admin mod.

Brand asset provenance is documented in `BRANDING.md`.
Upstream/fork lineage is documented in `UPSTREAMS.md`.

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

## Licensing

OpenACAI-authored manager changes are licensed under `GPL-3.0-or-later`.

The original RedManager project is licensed under `Apache-2.0`; its license text is preserved in `LICENSES/Apache-2.0.txt`, and attribution is preserved in `THIRD_PARTY_NOTICES.md`.

The OpenACAI Loader packages managed by this app are built on BepInEx and RedLoader/SonsSdk components. Their LGPL notices are preserved by the loader package and called out in this manager's third-party notices.
