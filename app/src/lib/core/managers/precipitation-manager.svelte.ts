import { invoke } from '@tauri-apps/api/core';
import { get } from 'svelte/store';

import type { Location } from '$lib/core/models/location';
import type { Unsubscriber, Writable } from 'svelte/store';
import type { NowcastPrediction } from '$lib/core/models/nowcast-prediction';


export class PrecipitationManager {
    private configured: boolean = false;
    private unsubscribers: Array<Unsubscriber> = [];
    private apiKeyDataPlatform: Writable<string>;
    private location: Writable<Location>;

    public loading = $state<boolean>(false);
    public prediction = $state<NowcastPrediction | undefined>(undefined);

    constructor(apiKeyDataPlatform: Writable<string>, location: Writable<Location>) {
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
            this.get_nowcast_prediction();
        }
    }

    public async get_nowcast_prediction(): Promise<void> {
        if(!this.configured || this.loading) {
            return;
        }

        this.loading = true;
		const reponse: string = await invoke('get_nowcast_forecast', {
			apiKey: get(this.apiKeyDataPlatform),
			longitude: get(this.location).longitude,
			latitude: get(this.location).latitude
		});

		this.prediction = JSON.parse(reponse);
        this.loading = false;
	}
}
