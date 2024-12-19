<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Search } from 'lucide-svelte';
	import { Location } from '$lib/core/models/location';

	let location = $state('');
	let geocodeResponse = $state<Array<Location>>([]);

	async function geocode(event: Event) {
		event.preventDefault();
		const reponse: string = await invoke('geocode', { location: location });
		const parsed = JSON.parse(reponse);
		const locations = parsed.docs.map((location: any) => {
			return Location.fromJSON(location);
		});

		geocodeResponse = locations;
	}
</script>

{#snippet locationResult(location: Location)}
	<div class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800">
		<p>{location.displayName}</p>
	</div>
{/snippet}

<div class=" w-full text-center space-y-4 pt-4">
	<form class="mx-auto w-full max-w-md space-y-8" onsubmit={geocode}>
		<div class="input-group divide-surface-200-800 grid-cols-[auto_1fr_auto] divide-x">
			<div class="input-group-cell">
				<Search size={16} />
			</div>
			<input type="search" placeholder="Search..." bind:value={location} />
			<button class="btn preset-filled">Search</button>
		</div>
	</form>

	<main class="bg-primary flex-col space-y-4">
		{#if geocodeResponse.length > 0}
			<div class="flex-col space-y-4">
				{#each geocodeResponse as location}
					{@render locationResult(location)}
				{/each}
			</div>
		{/if}
	</main>
</div>
