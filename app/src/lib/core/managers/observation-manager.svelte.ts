import { invoke } from '@tauri-apps/api/core';
import { get } from 'svelte/store';

import type { Location } from '$lib/core/models/location';
import type { Unsubscriber, Writable } from 'svelte/store';
import type { Observation } from '$lib/core/models/observation';

export class ObservationManager {
    private configured: boolean = false;
    private unsubscribers: Array<Unsubscriber> = [];
    private apiKeyDataPlatform: Writable<string>;
    private location: Writable<Location>;

    public loading = $state<boolean>(false);
    public observation = $state<Observation | undefined>(undefined);

    constructor(apiKeyDataPlatform: Writable<string>, location: Writable<Location>) {
        console.log("ObservationManager constructor");
        this.apiKeyDataPlatform = apiKeyDataPlatform;
        this.location = location;
        this.setup();
    }

    private setup(): void {
        this.unsubscribers.push(this.apiKeyDataPlatform.subscribe(() => {
            this.settingsUpdated();
        }));

        this.unsubscribers.push(this.location.subscribe(() => {
            this.settingsUpdated();
        }));
    }

    public destroy(): void {
        this.unsubscribers.forEach((unsubscriber) => unsubscriber());
    }

    private settingsUpdated(): void {
        if(get(this.apiKeyDataPlatform) !== undefined && get(this.location) !== undefined) {
            this.configured = true;
            this.get_closest_observation();
        }
    }

    public async get_closest_observation(): Promise<void> {
        if(!this.configured || this.loading) {
            return;
        }

        this.loading = true;

		const reponse: string = await invoke('get_closest_observation', {
			apiKey: get(this.apiKeyDataPlatform),
			longitude: get(this.location).longitude,
			latitude: get(this.location).latitude
		});

		this.observation = JSON.parse(reponse);
        this.loading = false;
        console.log("ObservationManager get_closest_observation", this.observation);
	}
}
