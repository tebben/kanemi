<script lang="ts">
	import { app } from '$lib/app';

	$: apiKey = app.settingsManager.apiKey;
</script>

<div>
	<label class="label">
		<span class="label-text">API Key</span>
		<input class="input" type="text" placeholder="Input" bind:value={$apiKey} />
	</label>
</div>
