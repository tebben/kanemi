<script lang="ts">
	import { app } from '$lib/app';
	import { Tabs } from '@skeletonlabs/skeleton-svelte';

	import WeatherScreen from '$lib/windows/Weather.svelte';
	import SettingsScreen from '$lib/windows/Settings.svelte';

	let group = $state('weather');
	let settingsLoaded = $derived(app.settingsManager.loaded);
</script>

<div
	class="grid h-screen max-w-full grid-rows-[auto_1fr_auto] overflow-x-hidden bg-gradient-to-b from-[#000] via-[#000] to-[#1e254a]"
>
	{#if settingsLoaded}
		<div class="mt-4">
			<Tabs bind:value={group} fluid>
				{#snippet list()}
					<Tabs.Control value="weather">Weather</Tabs.Control>
					<Tabs.Control value="settings">Settings</Tabs.Control>
				{/snippet}
			</Tabs>
		</div>

		<main class="bg-primary flex-col space-y-4 p-4">
			{#if group === 'weather'}
				<WeatherScreen />
			{:else if group === 'settings'}
				<SettingsScreen />
			{/if}
		</main>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="spinner"></div>
		</div>
	{/if}
</div>
