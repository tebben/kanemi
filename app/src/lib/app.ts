import { SettingsManager } from "$lib/core/managers/settings-manager.svelte";
import { PrecipitationManager } from "$lib/core/managers/precipitation-manager.svelte";
import { ObservationManager } from "$lib/core/managers/observation-manager.svelte";
import { writable } from "svelte/store";

import type { Writable } from "svelte/store";

class App {
    public loaded: Writable<boolean>;
    public settingsManager!: SettingsManager;
    public precipitationManager!: PrecipitationManager;
    public observationManager!: ObservationManager;

    constructor() {
        this.loaded = writable<boolean>(false);
    }

    public async init(): Promise<void> {
        // Create a new instance of the SettingsManager and wait
        // until settings are loaded before proceeding
        this.settingsManager = new SettingsManager();
        await this.settingsManager.load();

        this.precipitationManager = new PrecipitationManager(
            this.settingsManager.settings.apiKeyKNMIOpenDataPlatform,
            this.settingsManager.settings.location
        );

        this.observationManager = new ObservationManager(
            this.settingsManager.settings.apiKeyKNMIOpenDataPlatform,
            this.settingsManager.settings.location
        );

        // Set loaded to true to indicate that the app is initialized
        this.loaded.set(true);
    }

    public destroy() {
        this.precipitationManager.destroy();
    }
}

export const app = new App();
