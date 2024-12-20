<script lang="ts">
	import { app } from '$lib/app';
	import { Chart, registerables } from 'chart.js';
	import 'chartjs-adapter-luxon';

	Chart.register(...registerables);

	type PredictionValue = {
		datetime: string;
		value: number;
	};

	let chart: Chart;
	let prediction = $derived(app.precipitationManager.prediction);
	let chartCanvas = $state<HTMLCanvasElement | undefined>(undefined);
	let hoverValue = $state<PredictionValue>({
		datetime: new Date().toISOString(),
		value: 0
	});

	$effect(() => {
		if (chartCanvas && prediction !== undefined) {
			const dataLength = prediction.values.length;
			const dataRulerLight = Array(dataLength).fill(0.1);
			const dataRulerModerate = Array(dataLength).fill(2.5);
			const dataRulerHeavy = Array(dataLength).fill(7.5);
			const borderRuler = 'rgba(255, 255, 255, 0.7)';

			if (chart) {
				chart.destroy();
			}

			chart = new Chart(chartCanvas, {
				type: 'line',
				data: {
					labels: prediction.values.map((v) => new Date(`${v.datetime}`)),
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
							borderColor: borderRuler,
							borderWidth: 1,
							pointRadius: 0
						},
						{
							data: dataRulerModerate,
							borderColor: borderRuler,
							borderWidth: 1,
							pointRadius: 0
						},
						{
							data: dataRulerHeavy,
							borderColor: borderRuler,
							borderWidth: 1,
							pointRadius: 0
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
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
					onHover: (e: any) => {
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

<div
	class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 w-full p-4 flex flex-col space-y-4"
>
	<h1 class="h5">Precipitation</h1>
	<!-- <div>
		Last update: {new Date(prediction?.datetime).toLocaleTimeString()}
	</div> -->
	<div class="flex justify-between mb-4">
		<p>{new Date(`${hoverValue.datetime}`).toLocaleTimeString()}</p>
		<p>{hoverValue.value} mm/hr</p>
	</div>

	<div class="h-[20rem] w-full">
		<canvas bind:this={chartCanvas}></canvas>
	</div>
</div>
