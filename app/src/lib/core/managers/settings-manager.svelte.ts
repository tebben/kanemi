import { Store } from '@tauri-apps/plugin-store'
import { writable } from 'svelte/store';

const keyApiKeyOpenDataPlatform = 'apiKeyOpenDataPlatform';

export class SettingsManager {
    private store!: Store;

    public loaded = $state<boolean>(false);
    public apiKey = writable<string | undefined>(undefined);

    constructor() {
        this.loadSettings();
    }

    private async loadSettings(): Promise<void> {
        this.store = await Store.load('settings.json');
        this.apiKey.set(await this.getSetting<string | undefined>(keyApiKeyOpenDataPlatform));
        this.apiKey.subscribe((apiKey) => {
            this.setApiKey(apiKey);
        });

        this.loaded = true;
    }

    private async getSetting<T>(key: string): Promise<T | undefined> {
        return (await this.store.get<{ value: T }>(key))?.value;
    }

    private async setSetting<T>(key: string, value: T): Promise<void> {
        await this.store.set(key, { value: value });
    }

    public async setApiKey(apiKey: string | undefined): Promise<void> {
        await this.setSetting(keyApiKeyOpenDataPlatform, apiKey);
    }
}
