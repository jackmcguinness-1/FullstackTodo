<script lang="ts">
    import { onMount } from "svelte";

    let {handleCredentialResponse} = $props();

    onMount(() => {
        // Initialize the Google Sign-In button
        google.accounts.id.initialize({
            client_id:
                "594337534921-3arlg5tpasb984jt980p1tgdbgeckg8o.apps.googleusercontent.com",
            callback: handleCredentialResponse,
        });

        const gSignInBtn = document.getElementById("g_id_signin");
        if (gSignInBtn) {
            google.accounts.id.renderButton(
                gSignInBtn,
                { type: "standard", theme: "outline", size: "large" },
            );
        }

        google.accounts.id.prompt(); // Show One Tap prompt automatically
    });
</script>

<div id="g_id_signin"></div>
