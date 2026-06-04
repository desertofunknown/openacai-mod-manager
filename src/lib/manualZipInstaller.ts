import { dialog } from "@tauri-apps/api";
import { BaseZipInstaller } from "./baseZipInstaller";
import { getDirectoryPath } from "./store";

export class ManualZipInstaller extends BaseZipInstaller {
    private readonly _dialogTitle: string;

    constructor(name: string, dialogTitle: string) {
        super(name);
        this._dialogTitle = dialogTitle;
    }

    public async install(): Promise<void> {
        const selected = await dialog.open({
            title: this._dialogTitle,
            multiple: false,
            directory: false,
            filters: [
                {
                    name: "Loader zip",
                    extensions: ["zip"]
                }
            ]
        });

        if (!selected || Array.isArray(selected)) {
            return;
        }

        await this.unzip(selected, await getDirectoryPath());
    }

    public getTargetVersion(): Promise<string | null> {
        return Promise.resolve("local zip");
    }
}

