<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Search } from 'lucide-svelte';
	import { Location } from '$lib/core/models/location';
	import { ChevronRight, TriangleAlert } from 'lucide-svelte';

	let { setting } = $props();
	let value = setting.store;
	let locationSearch = $state('');
	let geocodeResponse = $state<Array<Location>>([]);
	let errorMessage = $state<string | undefined>(undefined);

	async function geocode(event: Event) {
		event.preventDefault();
		errorMessage = undefined;

		const reponse: string = await invoke('geocode', { location: locationSearch });
		const parsed = JSON.parse(reponse);
		const locations = parsed.docs.map((location: any) => {
			return Location.fromJSON(location);
		});

		if (locations.length === 0) {
			errorMessage = 'No locations found';
		}

		geocodeResponse = locations;
	}

	function selectLocation(location: Location) {
		value.set(location);
	}
</script>

{#if $value === undefined}
	<div class="w-full">No location set</div>
{:else}
	<dl class="space-y-1">
		<dt class="font-bold">Selected location</dt>
		<dd class="opacity-60">{$value.displayName}</dd>
	</dl>
{/if}

<div
	class="card preset-filled-surface-300-700 border-[1px] border-surface-400-600 w-full space-y-4 pt-4"
>
	<form class="mx-auto w-full max-w-md space-y-8" onsubmit={geocode}>
		<div
			class="input-group preset-filled-surface-200-800 divide-surface-400-600 grid-cols-[auto_1fr_auto] divide-x"
		>
			<div class="input-group-cell">
				<Search size={16} />
			</div>
			<input type="search" placeholder="Search for a location..." bind:value={locationSearch} />
			<button class="btn preset-filled">Search</button>
		</div>
	</form>

	<main class="bg-primary flex-col space-y-4">
		{#if geocodeResponse.length > 0}
			<div class="table-wrap text-left px-4">
				<table class="table caption-bottom">
					<caption class="pt-4">PDOK Locatieserver results</caption>
					<thead>
						<tr>
							<th class="w-10 pr-4">Type</th>
							<th class="w-full">Name</th>
							<th class="w-10 !text-right"></th>
						</tr>
					</thead>
					<tbody class="hover:[&>tr]:preset-tonal-primary">
						{#each geocodeResponse as location}
							<tr class="cursor-pointer" onclick={() => selectLocation(location)}>
								<td class="opacity-80"
									><button type="button" class="chip preset-tonal-tertiary min-w-28"
										>{location.type}</button
									></td
								>
								<td>{location.displayName}</td>
								<td class="w-10 !text-right"><ChevronRight size={24} /></td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}

		{#if errorMessage !== undefined}
			<div
				class="card preset-filled-surface-200-800 preset-outlined-warning-500 grid grid-cols-1 items-center gap-4 m-4 p-4 lg:grid-cols-[auto_1fr_auto]"
			>
				<TriangleAlert />
				<div>
					<p class="font-bold">Warning</p>
					<p class="type-scale-1 opacity-60">{errorMessage}</p>
				</div>
			</div>
		{/if}
	</main>
</div>
