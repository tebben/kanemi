<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import '../app.css';
	import { app } from '$lib/app';
	let { children } = $props();

	onMount(() => {
		app.init();
	});

	onDestroy(() => {
		app.destroy();
	});
</script>

{@render children()}
