<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let accounts = $state<string[]>([]);
    let currentAccount = $state<string | null>(null);
    let messageText = $state<string>("");

    onMount(async () => {
        await loadAccounts();
        await loadCurrentAccount();
    });

    const loadAccounts = async () => {
        try {
            accounts = await invoke<string[]>('accountget');
        } catch (error) {
            console.error('Failed to load accounts:', error);
            accounts = [];
        }
    };

    const loadCurrentAccount = async () => {
        try {
            currentAccount = await invoke<string | null>('accountgetcurrent');
        } catch (error) {
            console.error('Failed to load current account:', error);
            currentAccount = null;
        }
    };

    const setCurrentAccount = async (username: string) => {
        try {
            await invoke('accountsetcurrent', { username });
            currentAccount = username;
            messageText = `Account set to: ${username}`;
            setTimeout(() => messageText = "", 2000);
        } catch (error) {
            messageText = `Error: ${error}`;
            setTimeout(() => messageText = "", 2000);
        }
    };

    const deleteAccount = async (username: string) => {
        if (currentAccount === username) {
            messageText = "Cannot delete the currently selected account";
            setTimeout(() => messageText = "", 2000);
            return;
        }

        try {
            await invoke('accountdelete', { username });
            messageText = `Account deleted: ${username}`;
            await loadAccounts();
            setTimeout(() => messageText = "", 2000);
        } catch (error) {
            messageText = `Error: ${error}`;
            setTimeout(() => messageText = "", 2000);
        }
    };
</script>

<main>
    <div class="flex flex-row p-3">
        <ol class="w-full flex flex-col gap-5">
            {#if messageText}
                <div class="text-white font-roboto text-lg p-4 bg-neutral-800 rounded-2xl">
                    {messageText}
                </div>
            {/if}

            {#each accounts as username (username)}
                <li class="w-full flex flex-row text-white font-roboto text-2xl font-medium py-1 px-10 bg-neutral-800 rounded-2xl justify-between shadow-lg/60">
                    <span class="p-5 text-shadow-lg flex items-center gap-3">
                        {username}
                        {#if currentAccount === username}
                            <span class="text-sm bg-green-500 text-white px-3 py-1 rounded-lg">Active</span>
                        {/if}
                    </span>
                    <div class="flex flex-1 gap-5 m-auto place-content-end align-baseline ml-auto items-center">
                        <button 
                            onclick={() => setCurrentAccount(username)}
                            class="text-shadow-lg text-xl text-white px-10 py-5 font-roboto font-medium p-2 ease-in-out transition-all duration-500 bg-neutral-900 hover:bg-green-500 active:bg-green-900 hover:scale-105"
                        >
                            Set
                        </button>
                        <button 
                            onclick={() => deleteAccount(username)}
                            disabled={currentAccount === username}
                            class="text-shadow-lg text-xl text-white px-10 py-5 font-roboto font-medium p-2 bg-neutral-900 ease-in-out transition-all duration-500 hover:bg-red-500 active:bg-red-900 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            Delete
                        </button>
                    </div>
                </li>
            {/each}
        </ol>
    </div>
</main>