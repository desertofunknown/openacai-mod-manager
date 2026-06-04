<script lang="ts">
    import { onMount } from "svelte";
    import { dialog, shell } from "@tauri-apps/api";
    import SvgSpinnersBlocksWave from "~icons/svg-spinners/blocks-wave";
    import {
        clearNexusApiKey,
        fetchNexusSotfMods,
        getNexusModDownloadUrl,
        getNexusModPageUrl,
        getNexusSession,
        saveNexusApiKey,
        type NexusMod,
        type NexusSession,
        type NexusView
    } from "../lib/nexus";
    import {
        describeInstallSource,
        findMatchingInstall,
        readVortexDeployment,
        scanInstalledInventory,
        type InstalledInventoryEntry
    } from "../lib/modInventory";
    import { getDirectoryPath, isPathValid } from "../lib/store";

    let session: NexusSession = { is_connected: false };
    let apiKey = "";
    let mods: NexusMod[] = [];
    let inventory: InstalledInventoryEntry[] = [];
    let selectedView: NexusView = "trending";
    let isLoading = false;
    let status = "";
    let vortexStagingPath: string | null = null;

    onMount(async () => {
        await refreshInventory();
        await refreshSession();

        if (session.is_connected) {
            await loadMods();
        }
    });

    async function refreshSession() {
        try {
            session = await getNexusSession();
        } catch (error) {
            const message = `${error}`;
            session = {
                is_connected: false,
                error: message.includes("__TAURI_IPC__") ? undefined : message
            };
        }
    }

    async function refreshInventory() {
        if (!$isPathValid) {
            return;
        }

        inventory = await scanInstalledInventory();
        const vortex = await readVortexDeployment(await getDirectoryPath());
        vortexStagingPath = vortex.stagingPath;
    }

    async function connect() {
        if (!apiKey.trim()) {
            await dialog.message("Paste a Nexus Mods API key before connecting.", {
                title: "Nexus account",
                type: "info"
            });
            return;
        }

        isLoading = true;
        status = "Validating Nexus account...";

        try {
            session = await saveNexusApiKey(apiKey);
            apiKey = "";
            await loadMods();
        } catch (error) {
            await dialog.message(`${error}`, {
                title: "Nexus account error",
                type: "error"
            });
        } finally {
            status = "";
            isLoading = false;
        }
    }

    async function disconnect() {
        await clearNexusApiKey();
        session = { is_connected: false };
        mods = [];
    }

    async function loadMods(view: NexusView = selectedView) {
        selectedView = view;
        if (!session.is_connected) {
            return;
        }

        isLoading = true;
        status = "Loading Nexus mods...";

        try {
            await refreshInventory();
            const response = await fetchNexusSotfMods(selectedView);
            mods = response.mods;
            session = {
                ...session,
                rate_limit: response.rate_limit
            };
        } catch (error) {
            await dialog.message(`${error}`, {
                title: "Nexus Mods error",
                type: "error"
            });
        } finally {
            status = "";
            isLoading = false;
        }
    }

    async function openApiKeys() {
        await shell.open("https://www.nexusmods.com/users/myaccount?tab=api%20access");
    }

    async function openNexusGame() {
        await shell.open("https://www.nexusmods.com/games/sonsoftheforest");
    }

    async function openVortexStaging() {
        if (vortexStagingPath) {
            await shell.open(vortexStagingPath);
        }
    }

    async function openModPage(mod: NexusMod) {
        await shell.open(getNexusModPageUrl(mod));
    }

    async function openDownloadPage(mod: NexusMod) {
        await shell.open(getNexusModDownloadUrl(mod));
    }

    function installedMatch(mod: NexusMod): InstalledInventoryEntry | null {
        return findMatchingInstall(inventory, mod.name, mod.mod_id, [
            mod.author ?? "",
            mod.uploaded_by ?? ""
        ]);
    }

    function viewLabel(view: NexusView): string {
        switch (view) {
            case "latest_added":
                return "Latest";
            case "latest_updated":
                return "Updated";
            default:
                return "Trending";
        }
    }

    function formatTimestamp(value?: number, fallback?: string): string {
        if (value) {
            return new Date(value * 1000).toLocaleDateString();
        }

        return fallback ? new Date(fallback).toLocaleDateString() : "-";
    }
</script>

<div class="column nexus-page">
    <section class="account-panel">
        <div class="account-copy">
            <span class="panel-title">Vortex / Nexus Mods</span>
            {#if session.is_connected}
                <span class="panel-subtitle">Connected as {session.user?.name ?? "Nexus user"}</span>
            {:else}
                <span class="panel-subtitle">Connect a Nexus Mods API key to inspect Nexus-hosted Sons Of The Forest mods.</span>
            {/if}
        </div>

        {#if session.is_connected}
            <div class="account-actions">
                <button on:click={openNexusGame}>Open Nexus</button>
                {#if vortexStagingPath}
                    <button on:click={openVortexStaging}>Open Vortex Staging</button>
                {/if}
                <button class="uninstall" on:click={disconnect}>Disconnect</button>
            </div>
        {:else}
            <div class="connect-row">
                <input class="generic-input key-input" bind:value={apiKey} type="password" placeholder="Nexus API key" />
                <button class="install" on:click={connect}>Connect</button>
                <button on:click={openApiKeys}>API Keys</button>
            </div>
        {/if}
    </section>

    {#if session.error}
        <div class="notice warning">{session.error}</div>
    {/if}

    {#if session.rate_limit}
        <div class="rate-row">
            <span>Hourly {session.rate_limit.hourly_remaining ?? "-"} / {session.rate_limit.hourly_limit ?? "-"}</span>
            <span>Daily {session.rate_limit.daily_remaining ?? "-"} / {session.rate_limit.daily_limit ?? "-"}</span>
        </div>
    {/if}

    {#if session.is_connected}
        <div class="row-center controls">
            <button class="btn-left cat-btn" class:cat-btn-selected={selectedView === "trending"} on:click={() => loadMods("trending")}>{viewLabel("trending")}</button>
            <button class="cat-btn middle-btn" class:cat-btn-selected={selectedView === "latest_added"} on:click={() => loadMods("latest_added")}>{viewLabel("latest_added")}</button>
            <button class="btn-right cat-btn" class:cat-btn-selected={selectedView === "latest_updated"} on:click={() => loadMods("latest_updated")}>{viewLabel("latest_updated")}</button>
        </div>

        <div class="nexus-scroller">
            {#each mods as mod}
                {@const match = installedMatch(mod)}
                <article class="nexus-card">
                    <img
                        class="nexus-img"
                        src={mod.picture_url ?? "https://placehold.co/320x180/252525/FFF?text=Nexus"}
                        alt={mod.name}
                    />
                    <div class="nexus-body">
                        <div class="card-head">
                            <span class="mod-title">{mod.name}</span>
                            <span class="source-pill" class:source-vortex={match?.installSource === "vortex"} class:source-native={match?.installSource === "native"} class:source-manual={match?.installSource === "manual"}>
                                {describeInstallSource(match)}
                            </span>
                        </div>
                        <span class="description-content">{mod.summary ?? ""}</span>
                        <div class="facts">
                            <span>Author: <b>{mod.author ?? mod.uploaded_by ?? "-"}</b></span>
                            <span>Version: <b>{mod.version ?? "-"}</b></span>
                            <span>Updated: <b>{formatTimestamp(mod.updated_timestamp, mod.updated_time)}</b></span>
                            <span>Downloads: <b>{mod.mod_downloads ?? "-"}</b></span>
                        </div>
                        {#if match}
                            <span class="match-detail">{match.loaderType === "bepinex-plugin" ? "BepInEx plugin" : "RedLoader package"} in {match.expectedLocation}</span>
                        {/if}
                        <div class="button-row">
                            <button on:click={() => openModPage(mod)}>Open Page</button>
                            <button on:click={() => openDownloadPage(mod)}>Files</button>
                        </div>
                    </div>
                </article>
            {/each}

            {#if isLoading}
                <div class="loading-line">
                    <SvgSpinnersBlocksWave />
                    <span>{status}</span>
                </div>
            {/if}
        </div>
    {:else}
        <div class="notice">Nexus account connection is required before this section can compare Vortex packages against native installs.</div>
    {/if}
</div>

<style>
    .nexus-page {
        gap: 1em;
        justify-content: flex-start;
    }

    .account-panel {
        align-items: center;
        background: #10161d;
        border: 1px solid rgba(183, 236, 252, 0.14);
        border-radius: 8px;
        display: flex;
        gap: 1em;
        justify-content: space-between;
        padding: 1em;
        text-align: left;
    }

    .account-copy {
        display: flex;
        flex-direction: column;
        min-width: 0;
    }

    .panel-title {
        color: #eefcff;
        font-size: 1.25em;
        font-weight: 700;
    }

    .panel-subtitle {
        color: #9eb0bf;
        font-size: 0.9em;
    }

    .account-actions,
    .connect-row,
    .button-row,
    .rate-row {
        align-items: center;
        display: flex;
        gap: 0.5em;
    }

    .connect-row {
        flex: 1;
        justify-content: flex-end;
    }

    .key-input {
        margin: 0;
        max-width: 24em;
        min-width: 12em;
        text-align: left;
        width: 100%;
    }

    .rate-row {
        color: #9eb0bf;
        font-size: 0.85em;
        justify-content: flex-end;
    }

    .notice {
        background: #151b22;
        border: 1px solid rgba(252, 252, 252, 0.08);
        border-radius: 8px;
        color: #aab8c5;
        padding: 0.8em 1em;
        text-align: left;
    }

    .warning {
        color: #fdc66d;
    }

    .controls {
        justify-content: flex-end;
    }

    .cat-btn {
        height: 2.7em;
        margin: 0;
        padding: 0;
        width: 7em;
    }

    .middle-btn {
        border-radius: 0;
    }

    .cat-btn-selected {
        background-color: #111;
        color: #38d68d;
    }

    .nexus-scroller {
        display: grid;
        gap: 1em;
        grid-template-columns: repeat(auto-fill, minmax(330px, 1fr));
        max-height: 68vh;
        overflow-y: auto;
        padding-right: 0.4em;
    }

    .nexus-card {
        background: #121212;
        border-bottom: 2px solid #333;
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        min-height: 0;
        overflow: hidden;
    }

    .nexus-img {
        aspect-ratio: 16 / 9;
        background: #252525;
        object-fit: cover;
        width: 100%;
    }

    .nexus-body {
        display: flex;
        flex: 1;
        flex-direction: column;
        gap: 0.6em;
        padding: 0.9em;
        text-align: left;
    }

    .card-head {
        align-items: flex-start;
        display: flex;
        gap: 0.6em;
        justify-content: space-between;
    }

    .mod-title {
        color: #d8e4ec;
        font-size: 1.05em;
        font-weight: 700;
        line-height: 1.25;
        min-width: 0;
    }

    .description-content {
        color: #8d99a5;
        display: -webkit-box;
        font-size: 0.9em;
        line-height: 1.35;
        margin: 0;
        min-height: 3.6em;
        overflow: hidden;
        -webkit-line-clamp: 3;
        -webkit-box-orient: vertical;
    }

    .facts {
        color: #87929d;
        display: grid;
        font-size: 0.82em;
        gap: 0.25em;
        grid-template-columns: 1fr 1fr;
    }

    .facts b {
        color: #b9c7d2;
        font-weight: 600;
    }

    .match-detail {
        color: #78d9f4;
        font-size: 0.82em;
        font-weight: 700;
    }

    .source-pill {
        background: #202832;
        border: 1px solid rgba(252, 252, 252, 0.08);
        border-radius: 6px;
        color: #bfc8d2;
        flex: 0 0 auto;
        font-size: 0.72em;
        font-weight: 700;
        line-height: 1.2;
        max-width: 12em;
        overflow: hidden;
        padding: 0.35em 0.55em;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .source-vortex {
        color: #78d9f4;
    }

    .source-native {
        color: #38d68d;
    }

    .source-manual {
        color: #fdc66d;
    }

    .button-row {
        margin-top: auto;
    }

    .button-row button {
        flex: 1;
        margin: 0;
    }

    .loading-line {
        align-items: center;
        color: #38d68d;
        display: flex;
        font-weight: 700;
        gap: 0.6em;
        justify-content: center;
        min-height: 4em;
    }

    @media (max-width: 780px) {
        .account-panel,
        .connect-row,
        .account-actions {
            align-items: stretch;
            flex-direction: column;
        }

        .nexus-scroller {
            grid-template-columns: 1fr;
        }
    }
</style>
