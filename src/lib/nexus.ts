import { invoke } from "@tauri-apps/api/tauri";

export type NexusUser = {
    user_id?: number;
    name?: string;
    profile_url?: string;
    is_premium?: boolean;
    is_supporter?: boolean;
};

export type NexusRateLimit = {
    hourly_limit?: string;
    hourly_remaining?: string;
    hourly_reset?: string;
    daily_limit?: string;
    daily_remaining?: string;
    daily_reset?: string;
};

export type NexusSession = {
    is_connected: boolean;
    user?: NexusUser;
    rate_limit?: NexusRateLimit;
    error?: string;
};

export type NexusMod = {
    mod_id: number;
    name: string;
    summary?: string;
    description?: string;
    version?: string;
    author?: string;
    uploaded_by?: string;
    picture_url?: string;
    category_name?: string;
    endorsement_count?: number;
    mod_downloads?: number;
    mod_unique_downloads?: number;
    created_timestamp?: number;
    updated_timestamp?: number;
    created_time?: string;
    updated_time?: string;
};

type RawNexusModsResponse = {
    mods: NexusMod[] | { data?: NexusMod[] };
    rate_limit: NexusRateLimit;
};

type NexusModsResponse = {
    mods: NexusMod[];
    rate_limit: NexusRateLimit;
};

export type NexusView = "trending" | "latest_added" | "latest_updated";

export async function getNexusSession(): Promise<NexusSession> {
    return await invoke<NexusSession>("nexus_get_session");
}

export async function saveNexusApiKey(apiKey: string): Promise<NexusSession> {
    return await invoke<NexusSession>("nexus_save_api_key", { apiKey });
}

export async function clearNexusApiKey(): Promise<void> {
    await invoke("nexus_clear_api_key");
}

export async function fetchNexusSotfMods(view: NexusView): Promise<NexusModsResponse> {
    const response = await invoke<RawNexusModsResponse>("nexus_fetch_sotf_mods", { view });
    return {
        ...response,
        mods: Array.isArray(response.mods) ? response.mods : response.mods.data ?? []
    };
}

export function getNexusModPageUrl(mod: NexusMod): string {
    return `https://www.nexusmods.com/sonsoftheforest/mods/${mod.mod_id}`;
}

export function getNexusModDownloadUrl(mod: NexusMod): string {
    return `${getNexusModPageUrl(mod)}?tab=files`;
}
