<script lang="ts">
	let { setting } = $props();

	let value = setting.store;
</script>

<div class="w-full preset-filled-surface-300-700">
	<input class="input" type="text" placeholder="Input" bind:value={$value} />
</div>
