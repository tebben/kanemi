import { Store } from '@tauri-apps/plugin-store';
import { writable, type Writable } from 'svelte/store';

import type { Setting } from '$lib/core/models/settings/setting';
import type { Component } from 'svelte';

import StringSettingComponent from '$lib/components/settings/StringSetting.svelte';

export class StringSetting implements Setting<string> {
    key: string;
    title: string;
    description: string;
    store: Writable<string | undefined>;
    component: Component<any>;

    constructor(key: string, title: string, description: string) {
        this.key = key;
        this.title = title;
        this.description = description;
        this.store = writable<string | undefined>(undefined);
        this.component = StringSettingComponent;
    }

    async load(store: Store): Promise<void> {
        const value = (await store.get<{ value: string }>(this.key))?.value;
        this.store.set(value);
    }

    async save(store: Store, value: string): Promise<void> {
        await store.set(this.key, { value });
    }
}
