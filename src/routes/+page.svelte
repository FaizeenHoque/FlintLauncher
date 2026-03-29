<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let version = $state("");
    let currentAccount = $state<string | null>(null);

    onMount(async () => {
        try {
            currentAccount = await invoke<string | null>('accountgetcurrent');
        } catch (error) {
            console.error('Failed to load current account:', error);
            currentAccount = null;
        }
    });

    function launch() {
        invoke("launchprocess", { version: version || "26.1" })
            .then(() => {
                console.log("Process launched successfully");
            })
            .catch((error) => {
                console.error("Error launching process:", error); 
            });
    }
</script>


<main>
    <div class="flex flex-row gap-7 text-xl p-4 font-roboto font-medium">

        <div>
           <h1 class="text-gray-400">Storage Used:</h1> 
           <h2 class="text-white">0 GB</h2>
        </div>

        <div>
            <h1 class="text-gray-400">Play Time:</h1>
            <h2 class="text-white">0 Hours</h2>
        </div>

        <div>
            <h1 class="text-gray-400">Last Played:</h1>
            <h2 class="text-white">0 Hours ago</h2>
        </div>

        <div>
           <h1 class="text-gray-400">Account Selected:</h1> 
           <h2 class="text-white">{currentAccount || 'None'}</h2>
        </div>

        <div>
           <h1 class="text-gray-400">Version Selected:</h1> 
           <select class="w-full bg-neutral-800 text-white font-rubik outline-none ring-0 focus:ring-0 focus:outline-none border-0">
             <option>TEST_VERSION</option>
           </select>
        </div>

    </div>

    <div>
        <button class="text-white text-xl font-roboto font-medium py-5 px-15 m-3 bg-green-400 rounded-2xl transition-all ease-in duration-300 hover:bg-green-500 hover:shadow-green-900 shadow-lg active:bg-green-900 cursor-pointer" onclick={launch}>Launch</button>
    </div>
</main>