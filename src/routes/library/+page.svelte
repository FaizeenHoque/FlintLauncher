<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    interface Version {
        id: string;
        version_type: string;
        url?: string;
        time?: string;
        release_time: string;
        sha1?: string;
        complianceLevel?: string;
        installed: boolean;
    }

    let versions: Version[] = $state([]);
    let selectedVersion: string = $state('');
    let dropdownOpen = $state(false);
    let installingVersion: string | null = $state(null);
    let installProgress = $state(0);
    let installStatus = $state('');
    let error = $state('');
    let loading = $state(true);

    async function onMount() {
        await loadVersions();
    }

    async function loadVersions() {
        try {
            loading = true;
            versions = await invoke('fetch_available_versions');
            // Sort by version number descending
            versions = versions.sort((a, b) => {
                const aNum = parseFloat(a.id.split('.').slice(0, 2).join('.'));
                const bNum = parseFloat(b.id.split('.').slice(0, 2).join('.'));
                return bNum - aNum;
            });
            loading = false;
        } catch (err) {
            error = `Failed to load versions: ${err}`;
            loading = false;
        }
    }

    async function installVersionHandler() {
        if (!selectedVersion) {
            error = 'Please select a version';
            return;
        }

        const version = versions.find(v => v.id === selectedVersion && v.installed);
        if (version) {
            error = 'Version already installed';
            return;
        }

        installingVersion = selectedVersion;
        installProgress = 0;
        installStatus = 'Starting installation...';
        error = '';
        dropdownOpen = false;

        try {
            installStatus = 'Downloading version...';
            installProgress = 25;

            await invoke('install_version', { version: selectedVersion });

            installStatus = 'Installation complete!';
            installProgress = 100;
            await loadVersions();
            
            setTimeout(() => {
                installingVersion = null;
                selectedVersion = '';
                installProgress = 0;
                installStatus = '';
            }, 2000);
        } catch (err) {
            error = `Installation failed: ${err}`;
            installingVersion = null;
            installProgress = 0;
            installStatus = '';
        }
    }

    async function deleteInstalledVersion(version: string) {
        if (!confirm(`Delete version ${version}?`)) return;

        try {
            await invoke('delete_version', { version });
            await loadVersions();
        } catch (err) {
            error = `Failed to delete version: ${err}`;
        }
    }

    function formatDate(dateStr: string) {
        if (!dateStr) return '';
        return new Date(dateStr).toLocaleDateString();
    }

    function selectVersion(versionId: string) {
        selectedVersion = versionId;
        dropdownOpen = false;
    }

    onMount();
</script>

<main class="h-screen w-full flex flex-col items-center justify-center p-4">
    <div class="flex flex-col gap-6 p-6 font-roboto bg-neutral-800 rounded-xl w-full max-w-2xl">
        
        {#if error}
        <div class="bg-red-900/30 border border-red-500 text-red-300 px-4 py-3 rounded-lg text-sm">
            {error}
            <button onclick={() => error = ''} class="ml-2 underline">Dismiss</button>
        </div>
        {/if}

        {#if installingVersion}
        <div class="bg-neutral-700 p-4 rounded-lg">
            <div class="text-green-400 font-bold mb-2">Installing {installingVersion}...</div>
            <div class="w-full bg-neutral-600 rounded-full h-2 mb-2">
                <div class="bg-green-400 h-2 rounded-full transition-all" style="width: {installProgress}%"></div>
            </div>
            <div class="text-gray-300 text-sm">{installStatus}</div>
        </div>
        {:else if loading}
        <div class="text-center text-gray-300">
            <div class="inline-block animate-spin">⟳</div> Loading versions...
        </div>
        {:else}
        <div class="flex flex-col gap-3">
            <div class="flex flex-col gap-1">
                <label class="text-green-400 text-xs uppercase tracking-widest font-bold">Select Version</label>
                <div class="relative">
                    <button
                        onclick={() => dropdownOpen = !dropdownOpen}
                        class="bg-neutral-900 text-white text-sm font-medium py-2 px-3 rounded-lg w-full flex items-center justify-between hover:bg-neutral-700 transition-all focus:ring-0 focus:outline-none">
                        {selectedVersion || 'Select Version'}
                        <i class="fi fi-rr-angle-down text-xs transition-transform {dropdownOpen ? 'rotate-180' : ''}"></i>
                    </button>

                    {#if dropdownOpen}
                    <div class="absolute bottom-full mb-1 w-full bg-neutral-900 rounded-lg shadow-lg z-50" style="max-height: 200px; overflow-y: auto; scrollbar-width: thin; scrollbar-color: #4ade80 #171717;">
                        {#each versions as version}
                        <button
                            onmousedown={() => selectVersion(version.id)}
                            class="w-full text-left px-3 py-2 text-sm text-gray-400 hover:text-green-400 hover:bg-neutral-700 transition-colors {selectedVersion === version.id ? 'text-green-400 bg-neutral-700' : ''} {version.installed ? 'opacity-50' : ''}">
                            {version.id} ({version.version_type}) {version.installed ? '✓' : ''}
                        </button>
                        {/each}
                    </div>
                    {/if}
                </div>
            </div>

            <button
                onclick={installVersionHandler}
                disabled={!selectedVersion || !!installingVersion}
                class="bg-green-400 text-neutral-900 font-bold text-sm py-2 rounded-lg w-full flex items-center justify-center gap-2 shadow-lg shadow-green-400/30 hover:bg-green-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed">
                <i class="fi fi-rr-download"></i> Download & Install
            </button>
        </div>
        {/if}

        <div class="border-t border-neutral-700 pt-4">
            <div class="text-green-400 text-xs uppercase tracking-widest font-bold mb-3">Installed Versions ({versions.filter(v => v.installed).length})</div>
            <div class="flex flex-col gap-2 max-h-64 overflow-y-auto">
                {#each versions.filter(v => v.installed) as version}
                <div class="bg-neutral-900 rounded-lg p-3 flex justify-between items-center">
                    <div>
                        <div class="text-white font-semibold">{version.id}</div>
                        <div class="text-gray-400 text-xs">{formatDate(version.release_time)}</div>
                    </div>
                    <button
                        onclick={() => deleteInstalledVersion(version.id)}
                        class="bg-red-900/30 text-red-400 hover:bg-red-900/50 px-3 py-1 rounded text-xs transition-colors">
                        Delete
                    </button>
                </div>
                {/each}
                {#if versions.filter(v => v.installed).length === 0}
                <div class="text-gray-400 text-sm text-center py-4">No versions installed</div>
                {/if}
            </div>
        </div>
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
    }
</style>