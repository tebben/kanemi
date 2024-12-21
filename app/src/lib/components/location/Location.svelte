<script lang="ts">
	import { app } from '$lib/app';

	let location = $derived(app.settingsManager.settings.location);
	let observation = $derived(app.observationManager.observation);
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

		{#if observation}
			<div class="h6 mt-4">
				<div class="flex flex-col">
					<div>{observation.station.name}</div>
					<div>{observation.station.ta} °C</div>
					<div>Wind dir: {observation.station.dd}</div>
					<div>Wind speed: {Math.round(observation.station.ff * 3.6)} km/h</div>
					<div>Humidity: {observation.station.rh}%</div>
					<div>Rain 24 hours: {observation.station.r24h} mm</div>
					<div>ww: {observation.station.nc}</div>
				</div>
			</div>
		{/if}
	{:else}
		<p>No location selected</p>
	{/if}
</div>
