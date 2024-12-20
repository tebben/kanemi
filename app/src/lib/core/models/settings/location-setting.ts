import { Store } from '@tauri-apps/plugin-store';
import { writable, type Writable } from 'svelte/store';

import type { Setting } from '$lib/core/models/settings/setting';
import type { Component } from 'svelte';
import { Location } from '$lib/core/models/location';

import LocationSettingComponent from '$lib/components/settings/LocationSetting.svelte';

export class LocationSetting implements Setting<Location> {
    key: string;
    title: string;
    description: string;
    store: Writable<Location | undefined>;
    component: Component<any>;

    constructor(key: string, title: string, description: string) {
        this.key = key;
        this.title = title;
        this.description = description;
        this.store = writable<Location | undefined>(undefined);
        this.component = LocationSettingComponent;
    }

    async load(store: Store): Promise<void> {
        const value = (await store.get<{ value: string }>(this.key))?.value;
        if (!value) {
            return;
        }

        const location = Location.fromStore(JSON.parse(value));
        this.store.set(location);
    }

    async save(store: Store, value: Location): Promise<void> {
        const storeValue = value ? JSON.stringify(value) : undefined;
        await store.set(this.key, { value: storeValue });
    }
}
