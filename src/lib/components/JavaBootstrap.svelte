<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	interface Props {
		onComplete?: () => void;
		components?: string[] | null;
	}

	let { onComplete = () => {}, components = null } = $props();

	let progress: number = $state(0);
	let logs: string[] = $state([]);
	let currentComponent: string = $state('');
	let totalComponents: number = $state(0);
	let completedComponents: number = $state(0);
	let error: string | null = $state(null);
	let isComplete: boolean = $state(false);
	let isRetrying: boolean = $state(false);
	let showDetails: boolean = $state(false);

	onMount(async () => {
		try {
			// Listen to bootstrap start event
			const unlistenStart = await listen<{ total_components: number }>(
				'bootstrap:start',
				(event) => {
					totalComponents = event.payload.total_components;
					completedComponents = 0;
					progress = 0;
					error = null;
					logs = [
						`[bootstrap] Starting download of ${totalComponents} Java component(s)...`
					];
					console.log('[Bootstrap] Started:', event.payload);
				}
			);

			// Listen to progress events
			const unlistenProgress = await listen<{
				component: string;
				current_file: string;
				downloaded: number;
				total: number;
			}>('bootstrap:progress', (event) => {
				currentComponent = event.payload.component;
				const fileNum = event.payload.downloaded + 1;
				const totalFiles = event.payload.total > 0 ? event.payload.total : 1;

				// Calculate overall progress
				if (totalComponents > 0) {
					const componentProgress = totalFiles > 0 ? event.payload.downloaded / totalFiles : 0;
					progress = Math.round(
						((completedComponents + componentProgress) / totalComponents) * 100
					);
				}

				// Log progress
				const logMsg = `[${event.payload.component}] ${fileNum}/${totalFiles} - ${event.payload.current_file}`;
				logs = [...logs, logMsg];

				// Auto-scroll logs
				setTimeout(() => {
					const container = document.querySelector('[data-logs]');
					if (container) {
						container.scrollTop = container.scrollHeight;
					}
				}, 0);
			});

			// Listen to component done events
			const unlistenComponentDone = await listen<{ component: string }>(
				'bootstrap:component_done',
				(event) => {
					completedComponents++;
					const componentMsg = `[${event.payload.component}] Installation complete`;
					logs = [...logs, componentMsg];

					// Update progress to reflect completed component
					if (totalComponents > 0) {
						progress = Math.round((completedComponents / totalComponents) * 100);
					}

					console.log('[Bootstrap] Component done:', event.payload.component);
				}
			);

			// Listen to bootstrap complete event
			const unlistenDone = await listen('bootstrap:done', () => {
				progress = 100;
				logs = [...logs, '[bootstrap] All components installed successfully!'];
				isComplete = true;
				console.log('[Bootstrap] Completed successfully');

				// Auto-close after 1.5 seconds
				setTimeout(() => {
					onComplete();
				}, 1500);
			});

			// Listen to error events
			const unlistenError = await listen<{ message: string }>('bootstrap:error', (event) => {
				error = event.payload.message;
				const errorMsg = `[ERROR] ${event.payload.message}`;
				logs = [...logs, errorMsg];
				console.error('[Bootstrap] Error:', event.payload.message);
			});

			// Start the bootstrap process
			console.log('[Bootstrap] Invoking bootstrap_java_runtimes...');
			await invoke('bootstrap_java_runtimes', { components });
		} catch (err) {
			error = `Failed to start Java bootstrap: ${err}`;
			logs = [...logs, `[ERROR] ${error}`];
			console.error('[Bootstrap] Initialization error:', err);
		}
	});

	async function retry() {
		if (isRetrying) return;

		isRetrying = true;
		error = null;
		progress = 0;
		logs = ['[bootstrap] Retrying bootstrap process...'];
		completedComponents = 0;
		isComplete = false;

		try {
			console.log('[Bootstrap] Retrying...');
			await invoke('bootstrap_java_runtimes', { components });
		} catch (err) {
			error = `Retry failed: ${err}`;
			logs = [...logs, `[ERROR] ${error}`];
			console.error('[Bootstrap] Retry error:', err);
			isRetrying = false;
		}
	}
</script>

<!-- Full-screen centered bootstrap overlay -->
<div class="fixed inset-0 bg-linear-to-br from-neutral-950 via-neutral-900 to-neutral-950 flex items-center justify-center p-4 z-50">
	<div class="w-full max-w-2xl space-y-8">
		<!-- Header Section -->
		<div class="text-center space-y-4">
			<div class="flex justify-center">
				<div
					class="text-7xl font-bold text-green-400 font-roboto tracking-wider drop-shadow-2xl"
				>
					F
				</div>
			</div>
			<div>
				<p class="text-gray-200 text-lg font-semibold">Flint Launcher</p>
				<p class="text-gray-500 text-sm mt-1">Initializing Java Runtime Environment</p>
			</div>
		</div>

		<!-- Title -->
		<h2 class="text-3xl font-bold text-center text-white">Installing Java Runtime</h2>

		<!-- Error State -->
		{#if error}
			<div class="bg-red-950 bg-opacity-40 border border-red-500 border-opacity-60 rounded-lg p-6 space-y-4">
				<p class="text-red-300 text-sm font-medium">{error}</p>
				<button
					onclick={retry}
					disabled={isRetrying}
					class="w-full bg-red-600 hover:bg-red-700 disabled:bg-red-800 disabled:opacity-50 text-white font-medium py-3 px-4 rounded-lg transition-colors"
				>
					{isRetrying ? 'Retrying...' : 'Retry'}
				</button>
			</div>
		{/if}

		<!-- Progress Section -->
		{#if !isComplete && !error}
			<div class="bg-neutral-800 bg-opacity-40 border border-neutral-700 border-opacity-50 rounded-lg p-8 space-y-6">
				<!-- Progress Bar -->
				<div class="space-y-2">
					<div class="w-full bg-neutral-700 bg-opacity-40 rounded-full h-2 overflow-hidden">
						<div
							class="bg-linear-to-r from-green-500 to-green-400 h-full transition-all duration-300 shadow-lg shadow-green-500/30"
							style={`width: ${progress}%`}
						></div>
					</div>

					<!-- Progress Stats -->
					<div class="flex justify-between items-center">
						<span class="text-gray-400 text-xs font-medium">
							{completedComponents}/{totalComponents} components
						</span>
						<span class="text-green-400 font-bold text-sm">{progress}%</span>
					</div>
				</div>

				<!-- Current Component -->
				{#if currentComponent}
					<div class="text-center">
						<p class="text-gray-200 text-sm font-medium">
							Installing
							<span class="text-green-400">{currentComponent}</span>...
						</p>
					</div>
				{:else if totalComponents > 0}
					<div class="text-center">
						<p class="text-gray-300 text-sm">Preparing installation...</p>
					</div>
				{:else}
					<div class="text-center">
						<p class="text-gray-400 text-sm">Initializing...</p>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Success State -->
		{#if isComplete}
			<div class="bg-green-950 bg-opacity-40 border border-green-500 border-opacity-60 rounded-lg p-6 text-center">
				<p class="text-green-300 text-base font-semibold">
					✓ Java installation complete!
				</p>
			</div>
		{/if}

		<!-- Details Section -->
		<details bind:open={showDetails} class="bg-neutral-800 bg-opacity-30 border border-neutral-700 rounded-lg overflow-hidden">
			<summary class="cursor-pointer text-gray-300 text-sm font-medium hover:text-green-400 transition-colors flex items-center gap-3 p-4 select-none">
				<span class="inline-block transition-transform" style={`transform: rotate(${showDetails ? 180 : 0}deg)`}>
					▼
				</span>
				<span>Details Log</span>
			</summary>

			<!-- Log Output -->
			<div
				data-logs
				class="bg-neutral-900 bg-opacity-50 border-t border-neutral-700 px-4 py-3 max-h-96 overflow-y-auto text-xs font-mono text-gray-300 space-y-1"
			>
				{#each logs as log, i (i)}
					<div class="text-gray-400 wrap-break-word whitespace-pre-wrap">
						{log}
					</div>
				{/each}
				{#if logs.length === 0}
					<div class="text-gray-600">Initializing bootstrap logs...</div>
				{/if}
			</div>
		</details>

		<!-- Footer -->
		{#if isComplete}
			<p class="text-center text-gray-500 text-xs animate-pulse">Launching Flint Launcher...</p>
		{/if}
	</div>
</div>

<style>
	/* Hide default details marker */
	details summary::-webkit-details-marker {
		display: none;
	}

	/* Smooth animations */
	@keyframes fadeInUp {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.bootstrap-enter) {
		animation: fadeInUp 0.3s ease-out;
	}
</style>
