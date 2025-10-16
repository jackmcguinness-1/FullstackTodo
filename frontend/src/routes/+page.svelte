<script lang="ts">
    import GSignIn from "../components/GSignIn.svelte";
    let loginStatus: GLoginStatus = $state({loggedIn: false, user: null});

    function decodeJwt(jwt: string) {
        const payload = jwt.split(".")[1];
        return JSON.parse(atob(payload));
    }

    async function handleCredentialResponse(response: google.accounts.id.CredentialResponse) {
        const res = await fetch("http://localhost:8080/auth/oauth/google", {
            method: "POST",
            body: JSON.stringify({credential: response.credential})
        })
        //TODO: this line is only needed for debugging
        const user = decodeJwt(response.credential);
        loginStatus.loggedIn = true;
        loginStatus.user = user;
    }
</script>

<div>
    {#if loginStatus.loggedIn}
        welcome {loginStatus.user?.name}
    {:else}
        you are not yet logged in!
        <GSignIn {handleCredentialResponse}></GSignIn>
    {/if}
</div>