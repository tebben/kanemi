<script lang="ts">
	import { app } from '$lib/app';

	let location = $derived(app.settingsManager.settings.location);
	let municipality = $state<string | undefined>(undefined);
	let street = $state<string | undefined>(undefined);

	$effect(() => {
		if ($location?.municipality) {
			municipality = $location.municipality;
			return;
		}

		municipality = undefined;
	});

	$effect(() => {
		if ($location?.street) {
			if ($location.houseNumber) {
				street = `${$location.street}, ${$location.houseNumber}`;
			} else {
				street = $location.street;
			}

			return;
		}

		street = undefined;
	});
</script>

<div class="text-center">
	{#if $location}
		<div>
			{#if municipality}
				<div class="h4">
					{municipality}
				</div>
			{/if}

			{#if street}
				<div class="h6">
					{street}
				</div>
			{/if}
		</div>
	{:else}
		<p>No location selected</p>
	{/if}
</div>
