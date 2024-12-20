import type { Writable } from 'svelte/store';
import type { Store } from '@tauri-apps/plugin-store';
import type { Component } from 'svelte';

export interface Setting<T> {
    key: string;
    title: string;
    description: string;
    component: Component<any>;
    store: Writable<T | undefined>;
    load(store: Store): Promise<void>;
    save(store: Store, value: T | undefined): Promise<void>;
}
