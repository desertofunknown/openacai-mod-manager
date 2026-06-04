<script lang="ts">
    import { dialog, shell } from "@tauri-apps/api";
    import InstallFeature from "../lib/InstallationComponent.svelte";
    import PathSelector from "../lib/PathSelector.svelte";
    import { openAcaiLoaderFeature, melonFeature } from "../lib/featureInstaller";
    import { isPathValid, isDotnetInstalled, getDirectoryPath, processProgress, processing } from "../lib/store";
    import {
        ensureLatestOpenAcaiLoader,
        getInstalledLoaderVersion,
        verifyInstalledLoader,
        type LoaderIntegrityReport
    } from "../lib/openAcaiLoaderUpdater";
    import openAcaiMark from "/openacai-mark.svg";

    let features = [melonFeature, openAcaiLoaderFeature];
    let loaderReport: LoaderIntegrityReport | null = null;
    let loaderStatus = "Not checked";
    let loaderStatusClass = "manual";
    let checkedLoader = false;

    $: if ($isPathValid && !checkedLoader) {
        checkedLoader = true;
        refreshLoaderStatus();
    }

    async function openFolder() {
        await shell.open(await getDirectoryPath());
    }

    async function startGame() {
        const installedVersion = await getInstalledLoaderVersion();
        if (installedVersion) {
            processing.set(true);
            processProgress.set(0);

            try {
                loaderReport = await ensureLatestOpenAcaiLoader();
                updateLoaderStatus(loaderReport);
            } catch (error) {
                await dialog.message(`OpenACAI Loader could not be verified or updated:\n${error}`, {
                    title: "Loader update failed",
                    type: "error"
                });
                processing.set(false);
                return;
            }

            processing.set(false);
        }

        await shell.open("steam://rungameid/1326470");
    }

    async function refreshLoaderStatus() {
        try {
            loaderReport = await verifyInstalledLoader();
            updateLoaderStatus(loaderReport);
        } catch (error) {
            loaderStatus = `Loader update manifest unavailable: ${error}`;
            loaderStatusClass = "manual";
        }
    }

    async function repairLoader() {
        processing.set(true);
        processProgress.set(0);

        try {
            loaderReport = await ensureLatestOpenAcaiLoader();
            updateLoaderStatus(loaderReport);
            await dialog.message("OpenACAI Loader files are verified and current.", {
                title: "Loader verified",
                type: "info"
            });
        } catch (error) {
            await dialog.message(`${error}`, {
                title: "Loader repair failed",
                type: "error"
            });
        } finally {
            processing.set(false);
        }
    }

    function updateLoaderStatus(report: LoaderIntegrityReport) {
        if (!report.installedVersion) {
            loaderStatus = `Latest available: ${report.manifest.version} (${report.manifest.runtime.targetFramework}, .NET ${report.manifest.runtime.dotnetRuntimeVersion})`;
            loaderStatusClass = "manual";
            return;
        }

        if (report.needsUpdate) {
            loaderStatus = `${report.issues.length} file issue${report.issues.length === 1 ? "" : "s"} detected. Latest: ${report.manifest.version}. Installed: ${report.installedVersion}.`;
            loaderStatusClass = "update";
            return;
        }

        loaderStatus = `Verified ${report.installedVersion} (${report.manifest.runtime.targetFramework}, .NET ${report.manifest.runtime.dotnetRuntimeVersion})`;
        loaderStatusClass = "install";
    }
</script>

<div class="column">
    <section class="brand-panel">
        <a class="brand-link" href="https://gitlab.com/Godsring/openacai-loader" target="_blank" rel="noreferrer">
            <img class="brand-mark" src={openAcaiMark} alt="OpenACAI" />
            <span class="brand-copy">
                <span class="brand-title">OpenACAI Loader</span>
                <span class="brand-subtitle">Sons Of The Forest Mod Manager</span>
            </span>
        </a>
    </section>
    <PathSelector />
    {#if $isPathValid}
        {#each features as feature}
            <InstallFeature feature={feature} />
        {/each}
        <section class="loader-health">
            <span class="description-content">OpenACAI Loader integrity</span>
            <span class="health-status {loaderStatusClass}">{loaderStatus}</span>
            {#if loaderReport?.issues?.length}
                <span class="health-detail">{loaderReport.issues[0].path} - {loaderReport.issues[0].reason}</span>
            {/if}
            <div class="health-actions">
                <button class="tool-button" on:click={refreshLoaderStatus}>Check</button>
                <button class="tool-button install" on:click={repairLoader}>Verify / Repair</button>
            </div>
        </section>
    {/if}
    <br>
    <br>
    {#if $isPathValid}
        <button class="tool-button" on:click={openFolder}>Open Game Folder</button>
        <button class="tool-button" on:click={startGame}>Start Game</button>
    {/if}
    {#if !$isDotnetInstalled}
        <br>
        <span style="color: #a2a2a2;font-weight: 500"><a href="https://dotnet.microsoft.com/en-us/download/dotnet/10.0" target="_blank">.NET 10</a> is recommended for current OpenACAI tooling.</span>
    {/if}
</div>

<style>
    .brand-panel {
        display: flex;
        justify-content: center;
        margin: 0 auto 1.4em auto;
        width: min(100%, 520px);
    }

    .brand-link {
        align-items: center;
        background: #10161d;
        border: 1px solid rgba(183, 236, 252, 0.14);
        border-radius: 8px;
        display: flex;
        gap: 1em;
        justify-content: center;
        padding: 1em 1.2em;
        width: 100%;
    }

    .brand-mark {
        display: block;
        height: 92px;
        width: 92px;
        object-fit: contain;
    }

    .brand-copy {
        display: flex;
        flex-direction: column;
        line-height: 1.1;
        text-align: left;
    }

    .brand-title {
        color: #eefcff;
        font-size: 1.7em;
        font-weight: 700;
    }

    .brand-subtitle {
        color: #78d9f4;
        font-size: 0.9em;
        font-weight: 600;
        margin-top: 0.4em;
    }

    .tool-button {
        /* background: transparent; */
        /* border: 0; */
        background-color: #1e1e1e;
        color: #999;
        /* transition: background-color 0.25s; */
        /* color: #646cff; */
    }

    .loader-health {
        background: #10161d;
        border: 1px solid rgba(183, 236, 252, 0.14);
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        gap: 0.5em;
        margin: 0.8em auto;
        max-width: 520px;
        padding: 0.9em;
        text-align: left;
        width: min(100%, 520px);
    }

    .description-content {
        color: #a2a2a2;
        font-size: 0.9em;
        font-weight: 700;
    }

    .health-status {
        color: #b9c7d2;
        font-size: 0.9em;
        line-height: 1.35;
    }

    .health-detail {
        color: #fdc66d;
        font-size: 0.8em;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .health-actions {
        display: flex;
        gap: 0.5em;
    }

    .health-actions > button {
        flex: 1;
        margin: 0;
    }
</style>
