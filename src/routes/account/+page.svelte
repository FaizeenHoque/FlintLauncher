<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import { goto } from "$app/navigation";

    let accounts = $state<string[]>([]);
    let currentAccount = $state<string | null>(null);
    let message = $state<string>("");

    onMount(async () => {
        await loadAccounts();
        await loadCurrentAccount();
    });

    async function loadAccounts() {
        try {
            accounts = await invoke<string[]>('accountget');
        } catch (error) {
            console.error('Failed to load accounts:', error);
            accounts = [];
        }
    }

    async function loadCurrentAccount() {
        try {
            currentAccount = await invoke<string | null>('accountgetcurrent');
        } catch (error) {
            console.error('Failed to load current account:', error);
            currentAccount = null;
        }
    }

    async function setCurrentAccount(username: string) {
        try {
            await invoke('accountsetcurrent', { username });
            currentAccount = username;
            message = `Account set to: ${username}`;
            setTimeout(() => { message = ""; }, 2000);
        } catch (error) {
            message = `Error: ${error}`;
            setTimeout(() => { message = ""; }, 2000);
        }
    }

    async function deleteAccount(username: string) {
        if (currentAccount === username) {
            message = "Cannot delete the currently selected account";
            setTimeout(() => { message = ""; }, 2000);
            return;
        }

        try {
            await invoke('accountdelete', { username });
            message = `Account deleted: ${username}`;
            await loadAccounts();
            setTimeout(() => { message = ""; }, 2000);
        } catch (error) {
            message = `Error: ${error}`;
            setTimeout(() => { message = ""; }, 2000);
        }
    }

    function goToAddAccount() {
        window.location.href = "/account/addAccount";
    }
</script>

<main class="w-full min-h-screen bg-neutral-900">
    <div class="p-6 max-w-6xl mx-auto">
        <!-- Header -->
        <div class="mb-8">
            <h1 class="text-4xl font-bold text-white mb-2">Authenticated Accounts</h1>
            <p class="text-gray-400 text-sm">Manage all your identities across Mojang ecosystems. Switching accounts instantly updates your active profile settings.</p>
        </div>

        <!-- Accounts Display -->
        {#if message}
            <div class="mb-6 p-4 bg-neutral-700 border border-neutral-600 rounded-lg text-white text-sm font-roboto">
                {message}
            </div>
        {/if}

        {#if accounts.length === 0}
            <!-- No Accounts State -->
            <div class="bg-neutral-800 border-2 border-dashed border-neutral-600 rounded-xl p-12 text-center">
                <div class="mb-6">
                    <svg class="w-16 h-16 mx-auto text-gray-500 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m0 0h6M6 12a6 6 0 1112 0 6 6 0 01-12 0z"></path>
                    </svg>
                </div>
                <h2 class="text-2xl text-white font-bold mb-2">No Accounts Found</h2>
                <p class="text-gray-400 mb-6">You haven't added any accounts yet. Create your first account to get started.</p>
                <button 
                    onclick={goToAddAccount}
                    class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-8 rounded-lg transition-all duration-200 active:bg-green-700"
                >
                    + Add Your First Account
                </button>
            </div>
        {:else}
            <!-- Accounts Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
                {#each accounts as username (username)}
                    <div class="bg-neutral-800 rounded-xl p-6 border border-neutral-700 hover:border-green-500 transition-all duration-200">
                        <div class="flex items-center justify-between mb-4">
                            <h3 class="text-xl font-bold text-white truncate">{username}</h3>
                            {#if currentAccount === username}
                                <span class="text-xs bg-green-500 text-white px-2 py-1 rounded">Active</span>
                            {/if}
                        </div>
                        <div class="flex gap-2">
                            <button 
                                onclick={() => setCurrentAccount(username)}
                                class="flex-1 bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded-lg transition-all duration-200 active:bg-green-800 text-sm disabled:opacity-50"
                            >
                                Set
                            </button>
                            <button 
                                onclick={() => deleteAccount(username)}
                                disabled={currentAccount === username}
                                class="flex-1 bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-lg transition-all duration-200 active:bg-red-800 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                Delete
                            </button>
                        </div>
                    </div>
                {/each}
            </div>

            <!-- Add Account Button -->
            <div class="flex justify-center">
                <button 
                    onclick={goToAddAccount}
                    class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-8 rounded-lg transition-all duration-200 active:bg-green-700 flex items-center gap-2"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                    </svg>
                    Add Account
                </button>
            </div>
        {/if}
    </div>
</main>