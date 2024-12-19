import { LocationManager } from "./core/managers/location-manager.svelte";
import { SettingsManager } from "./core/managers/settings-manager.svelte";

class App {
    public settingsManager: SettingsManager;
    public locationManager: LocationManager;;

    constructor() {
        this.settingsManager = new SettingsManager();
        this.locationManager = new LocationManager();
    }

    public init() {}
}

export const app = new App();
