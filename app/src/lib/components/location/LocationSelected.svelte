<script lang="ts">
	import { app } from '$lib/app';

	let selectedLocation = app.locationManager.location;
</script>

<div>
	{#if selectedLocation}
		<p>{selectedLocation.displayName}</p>
	{:else}
		<p>No location selected</p>
	{/if}
</div>
