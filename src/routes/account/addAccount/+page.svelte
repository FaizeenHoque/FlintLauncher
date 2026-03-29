<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { ScriptElementKind } from "typescript";

    let accounts = $state("");
    let infoText: HTMLElement | null = null;
    let accountCount = $state(0);
    const MAX_ACCOUNTS = 6;

    onMount(() => {
        infoText = document.getElementById("infoText");
        loadAccountCount();
    });

    const loadAccountCount = async () => {
        try {
            const result = (await invoke('accountget')) as unknown;
            if (Array.isArray(result)) {
                accountCount = result.length;
            }
        } catch (error) {
            console.error('Failed to load accounts:', error);
        }
    };

    const addAccount = async () => {
        if (accountCount >= MAX_ACCOUNTS) {
            if (infoText) infoText.innerHTML = `Maximum ${MAX_ACCOUNTS} accounts allowed.`;
            return;
        }
        try {
            const result = (await invoke('accountcreate', { username: accounts })) as unknown;
            if (infoText && typeof result === 'string') {
                infoText.innerText = `Account created: ${result}`;
                accounts = "";
                accountCount++;
            }
        } catch (error) {
            console.error('Failed to add account:', error);
            if (infoText) infoText.innerHTML = 'Failed to add account.';
        }
    }

</script>

<main>
    <div class="flex flex-col p-5 gap-3 text-center place-items-center place-content-baseline justify-center justify-items-center h-screen">
        <div class="text-white text-2xl font-roboto">Username:</div>
        <input bind:value={accounts} class="bg-neutral-800 text-white font-roboto text-xl rounded-lg p-2 outline-none">
        <button onclick={addAccount} disabled={accountCount >= MAX_ACCOUNTS} class="text-white font-medium font-roboto p-2 px-5 text-centeer bg-neutral-800 rounded-xl transition-all hover:bg-green-500 active:bg-neutral-800 disabled:opacity-50 disabled:cursor-not-allowed">Add User</button>
        {#if accountCount >= MAX_ACCOUNTS}
            <div class="text-red-500 text-lg font-roboto font-bold">You cannot create more than {MAX_ACCOUNTS} accounts</div>
        {/if}
        <div class="text-white text-sm font-roboto" id="infoText"></div>
    </div>
</main>