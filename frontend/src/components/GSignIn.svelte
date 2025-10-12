<script lang="ts">
    import { onMount } from "svelte";

    function handleCredentialResponse(response: any) {
        // This is the JWT ID token you send to your backend
        console.log("Encoded JWT ID token: " + response.credential);
        const user = decodeJwt(response.credential);
        console.log("Full user info:", user);
        console.log("User name:", user.name);
        console.log("User email:", user.email);
        console.log("User picture:", user.picture);
    }

    function decodeJwt(jwt: string) {
        // JWT has three parts: header.payload.signature
        const payload = jwt.split(".")[1];
        return JSON.parse(atob(payload));
    }

    onMount(() => {
        // Initialize the Google Sign-In button
        google.accounts.id.initialize({
            client_id:
                "594337534921-3arlg5tpasb984jt980p1tgdbgeckg8o.apps.googleusercontent.com",
            callback: handleCredentialResponse,
        });

        google.accounts.id.renderButton(
            document.getElementById("g_id_signin"),
            { theme: "outline", size: "large" },
        );

        google.accounts.id.prompt(); // Show One Tap prompt automatically
    });
</script>

<div id="g_id_signin"></div>
