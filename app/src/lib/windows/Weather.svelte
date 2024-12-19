<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { app } from '$lib/app';
	import { get } from 'svelte/store';
	import { Chart, registerables } from 'chart.js';
	import 'chartjs-adapter-luxon';

	Chart.register(...registerables);

	type Prediction = {
		values: Array<PredictionValue>;
	};

	type PredictionValue = {
		datetime: string;
		value: number;
	};

	let chart: Chart;
	let apiKey = $derived(app.settingsManager.apiKey);
	let prediction = $state<Prediction | undefined>(undefined);
	let chartCanvas = $state<HTMLCanvasElement | undefined>(undefined);
	let hoverValue = $state<PredictionValue | undefined>(undefined);

	async function get_nowcast_prediction() {
		const key = get(apiKey);
		if (key === undefined) {
			return;
		}

		const reponse: string = await invoke('get_nowcast_forecast', {
			apiKey: key,
			longitude: 5.32144283,
			latitude: 51.68726598
		});
		prediction = JSON.parse(reponse);
	}

	onMount(() => {
		get_nowcast_prediction();
	});

	$effect(() => {
		if (chartCanvas && prediction !== undefined) {
			const dataLength = prediction.values.length;
			const dataRulerLight = Array(dataLength).fill(0.1);
			const dataRulerModerate = Array(dataLength).fill(2.5);
			const dataRulerHeavy = Array(dataLength).fill(7.5);

			chart = new Chart(chartCanvas, {
				type: 'line',
				data: {
					labels: prediction.values.map((v) => new Date(`${v.datetime}Z`)),
					datasets: [
						{
							label: 'mm/hr',
							data: prediction.values.map((v) => v.value),
							borderColor: 'rgba(255, 0, 128, 1)',
							backgroundColor: 'rgba(255, 0, 128, 0.5)',
							tension: 0.1,
							borderWidth: 2,
							pointRadius: 0,
							fill: true
						},
						{
							data: dataRulerLight,
							borderColor: 'rgba(255, 255, 255, 0.7)',
							borderWidth: 1,
							pointRadius: 0
						},
						{
							data: dataRulerModerate,
							borderColor: 'rgba(255, 255, 255, 0.7)',
							borderWidth: 1,
							pointRadius: 0
						},
						{
							data: dataRulerHeavy,
							borderColor: 'rgba(255, 255, 255, 0.7)',
							borderWidth: 1,
							pointRadius: 0
						}
					]
				},
				options: {
					responsive: true,
					interaction: {
						mode: 'index'
					},
					scales: {
						x: {
							type: 'time',
							time: {
								unit: 'minute',
								displayFormats: {
									minute: 'HH:mm'
								}
							},
							title: {
								display: false,
								text: 'Time'
							},
							ticks: {
								maxTicksLimit: 25,
								color: 'rgba(255, 255, 255, 0.9)'
							},
							grid: {
								display: false
							}
						},
						y: {
							title: {
								display: false,
								text: 'Value'
							},
							grid: {
								drawTicks: false,
								display: false
							},
							ticks: {
								color: 'rgba(255, 255, 255, 0.9)'
							}
						}
					},
					onHover: (e) => {
						const test = chart.getElementsAtEventForMode(e, 'index', { intersect: false }, true);
						if (test.length <= 0 || prediction === undefined) {
							return;
						}

						hoverValue = prediction.values[test[0].index];
					},
					plugins: {
						legend: {
							display: false
						}
					}
				}
			});
		}
	});
</script>

<div>
	<h1 class="h4">Precipitation</h1>
	<!-- 	<button type="button" onclick={get_nowcast_prediction} class="btn-icon preset-filled"
		>&rarr;</button
	> -->
</div>

<div class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 w-full p-4">
	{#if hoverValue !== undefined}
		<div class="flex justify-between mb-4">
			<p>{new Date(`${hoverValue.datetime}Z`).toLocaleTimeString()}</p>
			<p>{hoverValue.value} mm/hr</p>
		</div>
	{/if}

	<canvas bind:this={chartCanvas}></canvas>
</div>
