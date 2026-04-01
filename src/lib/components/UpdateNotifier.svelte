<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	interface UpdateInfo {
		current_version: string;
		latest_version: string;
		release_notes: string | null;
	}

	let showUpdateDialog = $state(false);
	let updateInfo = $state<UpdateInfo | null>(null);
	let isDownloading = $state(false);
	let downloadStatus = $state('');

	onMount(() => {
		if (browser) {
			checkForUpdates();
		}
	});

	async function checkForUpdates() {
		if (!browser) return;

		try {
			const { check } = await import('@tauri-apps/plugin-updater');
			const update = await check();

			if (update?.available) {
				updateInfo = {
					current_version: update.currentVersion,
					latest_version: update.version,
					release_notes: update.body
				};
				showUpdateDialog = true;
			}
		} catch (error) {
			console.error('Failed to check for updates:', error);
		}
	}

	async function handleDownloadAndInstall() {
		if (!updateInfo || !browser) return;

		isDownloading = true;
		downloadStatus = 'Downloading update...';

		try {
			const { check } = await import('@tauri-apps/plugin-updater');
			const update = await check();
			if (update?.available) {
				await update.downloadAndInstall();
				downloadStatus = 'Update installed. Restarting...';
				// The updater automatically restarts the app after install
			}
		} catch (error) {
			downloadStatus = `Error: ${error}`;
			isDownloading = false;
		}
	}

	function closeDialog() {
		showUpdateDialog = false;
		updateInfo = null;
	}
</script>

{#if showUpdateDialog && updateInfo}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
		<div class="bg-gray-900 rounded-lg p-6 max-w-md w-full mx-4 border border-green-400">
			<h2 class="text-xl font-bold text-white mb-4">Update Available</h2>

			<div class="space-y-4 text-gray-300">
				<div>
					<p class="text-sm opacity-75">Current Version</p>
					<p class="font-mono text-white">{updateInfo.current_version}</p>
				</div>

				<div>
					<p class="text-sm opacity-75">Latest Version</p>
					<p class="font-mono text-green-400">{updateInfo.latest_version}</p>
				</div>

				{#if updateInfo.release_name}
					<div>
						<p class="text-sm opacity-75">Release</p>
						<p class="text-white">{updateInfo.release_name}</p>
					</div>
				{/if}

				{#if updateInfo.release_notes}
					<div>
						<p class="text-sm opacity-75">Release Notes</p>
						<p class="text-white text-sm max-h-32 overflow-y-auto">
							{updateInfo.release_notes}
						</p>
					</div>
				{/if}

				{#if downloadStatus}
					<p class="text-sm text-yellow-400">{downloadStatus}</p>
				{/if}
			</div>

			<div class="flex gap-3 mt-6">
				<button
					onclick={closeDialog}
					disabled={isDownloading}
					class="flex-1 px-4 py-2 bg-gray-800 text-white rounded hover:bg-gray-700 disabled:opacity-50"
				>
					Later
				</button>
				<button
					onclick={handleDownloadAndInstall}
					disabled={isDownloading}
					class="flex-1 px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50"
				>
					{isDownloading ? 'Downloading...' : 'Update Now'}
				</button>
			</div>
		</div>
	</div>
{/if}
