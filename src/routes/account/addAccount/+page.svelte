<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let accounts = $state("");
    let infoText: HTMLElement | null = null;
    let accountCount = $state(0);
    let existingAccounts = $state<string[]>([]);
    let isLoading = $state(false);
    let validationError = $state("");
    const MAX_ACCOUNTS = 6;
    const USERNAME_MIN_LENGTH = 3;
    const USERNAME_MAX_LENGTH = 16;
    const VALID_USERNAME_PATTERN = /^[a-zA-Z0-9_-]*$/;

    onMount(() => {
        infoText = document.getElementById("infoText");
        loadAccountCount();
        loadExistingAccounts();
    });

    const validateUsername = (username: string): string => {
        const trimmed = username.trim();
        
        if (!trimmed) {
            return "Username cannot be empty.";
        }
        
        if (trimmed.length < USERNAME_MIN_LENGTH) {
            return `Username must be at least ${USERNAME_MIN_LENGTH} characters.`;
        }
        
        if (trimmed.length > USERNAME_MAX_LENGTH) {
            return `Username cannot exceed ${USERNAME_MAX_LENGTH} characters.`;
        }
        
        if (!VALID_USERNAME_PATTERN.test(trimmed)) {
            return "Username can only contain letters, numbers, underscores, and hyphens.";
        }
        
        if (existingAccounts.some(acc => acc.toLowerCase() === trimmed.toLowerCase())) {
            return "An account with this name already exists.";
        }
        
        return "";
    };

    const handleUsernameInput = () => {
        validationError = validateUsername(accounts);
    };

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

    const loadExistingAccounts = async () => {
        try {
            const result = (await invoke('accountget')) as unknown;
            if (Array.isArray(result)) {
                existingAccounts = result;
            }
        } catch (error) {
            console.error('Failed to load existing accounts:', error);
        }
    };

    const addAccount = async () => {
        const error = validateUsername(accounts);
        if (error) {
            if (infoText) infoText.innerHTML = `<span style="color: #ef4444;">${error}</span>`;
            return;
        }
        if (accountCount >= MAX_ACCOUNTS) {
            if (infoText) infoText.innerHTML = `<span style="color: #ef4444;">Maximum ${MAX_ACCOUNTS} accounts allowed.</span>`;
            return;
        }
        isLoading = true;
        try {
            const result = (await invoke('accountcreate', { username: accounts })) as unknown;
            if (infoText && typeof result === 'string') {
                infoText.innerHTML = `<span style="color: #22c55e; font-weight: bold;">✓ Account created successfully!</span>`;
                accounts = "";
                validationError = "";
                accountCount++;
                setTimeout(() => {
                    if (infoText) infoText.innerHTML = '';
                    window.location.href = '/account';
                }, 2000);
            }
        } catch (error: any) {
            console.error('Failed to add account:', error);
            const errorMessage = error.message || error.toString();
            if (infoText) infoText.innerHTML = `<span style="color: #ef4444; font-weight: bold;">✗ ${errorMessage}</span>`;
            isLoading = false;
        }
    }

</script>

<main class="w-full min-h-screen bg-neutral-900 flex items-center justify-center p-6">
    <div class="w-full max-w-md">
        <div class="bg-neutral-800 rounded-xl p-8 border border-neutral-700">
            <h1 class="text-3xl font-bold text-white mb-2 text-center">Add Account</h1>
            <p class="text-gray-400 text-center mb-8">Create a new offline account</p>

            <div class="mb-6">
                <label for="username" class="block text-white font-bold mb-3">Username</label>
                <input 
                    id="username"
                    bind:value={accounts}
                    onchange={handleUsernameInput}
                    oninput={handleUsernameInput}
                    placeholder="Enter username (3-16 characters)"
                    autocomplete="off"
                    class="w-full bg-neutral-700 text-white font-roboto text-lg rounded-lg p-3 outline-none focus:ring-2 focus:ring-green-500 transition-all duration-200" 
                    onkeydown={(e) => e.key === 'Enter' && addAccount()}
                >
                {#if validationError}
                    <p class="text-red-400 text-sm mt-2">{validationError}</p>
                {:else if accounts.trim()}
                    <p class="text-green-400 text-sm mt-2">✓ Username is valid</p>
                {/if}
                <p class="text-gray-500 text-xs mt-2">Allowed: Letters, numbers, underscores, hyphens</p>
            </div>

            <button 
                onclick={addAccount} 
                disabled={accountCount >= MAX_ACCOUNTS || isLoading} 
                class="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-all duration-200 active:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed mb-4"
            >
                {isLoading ? 'Creating...' : 'Create Account'}
            </button>

            {#if accountCount >= MAX_ACCOUNTS}
                <div class="bg-red-900/20 border border-red-500 text-red-300 px-4 py-3 rounded-lg text-sm font-roboto font-bold text-center mb-4">
                    You cannot create more than {MAX_ACCOUNTS} accounts
                </div>
            {/if}

            <div class="text-center font-roboto" id="infoText"></div>
        </div>
    </div>
</main>