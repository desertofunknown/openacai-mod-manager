<script lang="ts">
    import { shell } from "@tauri-apps/api";
    import InstallFeature from "../lib/InstallationComponent.svelte";
    import PathSelector from "../lib/PathSelector.svelte";
    import { openAcaiLoaderFeature, melonFeature } from "../lib/featureInstaller";
    import { isPathValid, isDotnetInstalled, getDirectoryPath } from "../lib/store";
    import openAcaiMark from "/openacai-mark.svg";

    let features = [melonFeature, openAcaiLoaderFeature];

    async function openFolder() {
        await shell.open(await getDirectoryPath());
    }

    async function startGame() {
        await shell.open("steam://rungameid/1326470");
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
    {/if}
    <br>
    <br>
    {#if $isPathValid}
        <button class="tool-button" on:click={openFolder}>Open Game Folder</button>
        <button class="tool-button" on:click={startGame}>Start Game</button>
    {/if}
    {#if !$isDotnetInstalled}
        <br>
        <span style="color: #a2a2a2;font-weight: 500"><a href="https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/runtime-desktop-6.0.21-windows-x64-installer" target="_blank">Dotnet 6</a> is needed for the loader to work!</span>
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
</style>
