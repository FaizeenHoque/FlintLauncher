<script>
    import { createAccount } from "../lib/account";

    let username = $state("");

    async function handleConfirm() {
        try {
            await createAccount(username);
            console.log("done");
        } catch (e) {
            console.error("failed:", e);
        }
    }
</script>

<main>
    
</main>``