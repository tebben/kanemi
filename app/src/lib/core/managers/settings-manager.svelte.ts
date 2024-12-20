import { Store } from '@tauri-apps/plugin-store'
import { StringSetting } from '$lib/core/models/settings/string-setting';
import { LocationSetting } from '$lib/core/models/settings/location-setting';

import type { Setting } from '$lib/core/models/settings/setting';
import type { Writable } from 'svelte/store';

const keyApiKNMIOpenDataPlatform = 'apiKeyKNMIOpenDataPlatform';
const keyApiKNMINotification = 'apiKeyKNMINotification';
const keyLocation = 'location';

export class SettingsManager {
    private store!: Store;

    public loaded = $state<boolean>(false);
    public settings: Record<string, Writable<any | undefined>> = {};
    public settingsMap = $state<Record<string, Setting<any>>>({});

    constructor() {}

    public async load(): Promise<void> {
        await this.loadStore();
    }

    private addSetting<T>(setting: Setting<T>): void {
        this.settingsMap[setting.key] = setting;
        this.settings[setting.key] = setting.store;

        setting.store.subscribe(async (value) => {
            if (!this.loaded) {
                return;
            }

            await setting.save(this.store, value);
        });
    }

    private async loadStore(): Promise<void> {
        this.store = await Store.load('settings.json');
        this.addSetting(new StringSetting(keyApiKNMIOpenDataPlatform, 'API Key Open Data Platform', 'The API key to access the Open Data Platform'));
        this.addSetting(new StringSetting(keyApiKNMINotification, 'API Key KNMI Notification', 'The API key to access the KNMI notification service'));
        this.addSetting(new LocationSetting(keyLocation, 'Location', 'The location to use for weather data'));

        for (const key in this.settingsMap) {
            const setting = this.settingsMap[key];
            await setting.load(this.store);
        }

        this.loaded = true;
    }
}
