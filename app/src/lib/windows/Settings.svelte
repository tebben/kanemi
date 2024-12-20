<script lang="ts">
	import { app } from '$lib/app';
	import { Modal } from '@skeletonlabs/skeleton-svelte';
	import { ChevronRight } from 'lucide-svelte';

	import type { Setting } from '$lib/core/models/settings/setting';

	let modalOpen = $state(false);
	let settings = $derived(Object.values(app.settingsManager.settingsMap));
	let selectedSetting = $state<Setting<any> | undefined>(undefined);

	function modalClose() {
		modalOpen = false;
	}

	function openSettingsModal<T>(setting: Setting<T>) {
		selectedSetting = setting;
		modalOpen = true;
	}
</script>

<h1 class="h4">Settings</h1>

{#each settings as setting}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		onclick={() => {
			openSettingsModal(setting);
		}}
		class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 w-full p-3 cursor-pointer"
	>
		<div class="flex justify-between items-center">
			<dl class="space-y-1">
				<dt class="font-bold">{setting.title}</dt>
				<dd class="opacity-60">{setting.description}</dd>
			</dl>
			<ChevronRight size={24} />
		</div>
	</div>
{/each}

<Modal
	bind:open={modalOpen}
	triggerBase="btn preset-tonal"
	contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl min-w-[80%] max-h-[80%] overflow-x-hidden overflow-y-hidden"
	backdropClasses="backdrop-blur-sm"
>
	{#snippet content()}
		<div class="flex-col space-y-4 h-full overflow-hidden">
			{#if selectedSetting !== undefined}
				<div>
					<header class="flex-col">
						<div class="h5">{selectedSetting.title}</div>
						<div class="opacity-60 mt-2">{selectedSetting.description}</div>
					</header>
				</div>

				<hr class="hr" />

				<div class="max-h-[50vh] overflow-auto space-y-4">
					<selectedSetting.component setting={selectedSetting}></selectedSetting.component>
				</div>

				<footer class="flex justify-end gap-4">
					<button type="button" class="btn preset-filled" onclick={modalClose}>Close</button>
				</footer>
			{/if}
		</div>
	{/snippet}
</Modal>
