<script lang="ts">
    import { ApiClient } from "../../api/apiClient";
    import { token, activeSection, message } from "../../store";
    import AlertBox from "../components/AlertBox.svelte";
    import { Section } from "../Sections";
    import {_} from "svelte-i18n"

    let error: boolean = false
    let username = ""
    let password = ""

    async function login(){
        let usrToken = await ApiClient.logIn(username, password)
        if(usrToken.access_token){
            message.set($_('login.success'))
            token.set(usrToken.access_token)
            activeSection.set(Section.CLOCKIN)
        }
        else{
            error = true
        }
    }
</script>

<div class="section">
    <div class="card">
        <p class="m-auto text-3xl mb-4">{$_('general.login')}</p>
        <div class="flex flex-col mb-4">
            <input type="text" placeholder="{ $_('user.username') }" bind:value={username} class="text-input"/>
            <input type="password" placeholder="{ $_('user.password') }" bind:value={password} class="text-input"/>
        </div>
        <button on:click={login} class="button">{ $_('general.login') }</button>
        {#if error}
            <div class="mt-4">
                <AlertBox message="{$_('login.error')}"/>
            </div>
        {/if}
    </div>
</div>
