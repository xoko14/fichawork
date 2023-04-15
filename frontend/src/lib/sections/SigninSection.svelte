<script lang="ts">
    import { activeSection, message } from "../../store";
    import { ApiClient } from "../../api/apiClient";
    import { User, UserCreate } from "../../api/models/User";
    import AlertBox from "../components/AlertBox.svelte";
    import { Section } from "../Sections";
    import { _ } from 'svelte-i18n'


    let username: string
    let name: string
    let password: string
    let rPassword: string

    let error: boolean = false;
    let errorMessage: string
    let verified: boolean = false

    function verifyUsername(): boolean{
        if(!/^[\w\d\-_]{1,25}$/.test(username?? "")){
            errorMessage = $_('signin.usernameverification')
            return false
        }
        return true
    }

    function verifyName(): boolean{
        if(!/.{1,100}/.test(name?? "")){
            errorMessage = $_('signin.nameverification')
            return false
        }
        return true
    }

    function verifyPassword(): boolean{
        if(!/^(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*[*.!@$%^&(){}\[\]:;<>,.?\/~_+-=|\\]).{8,32}$/.test(password?? "")){
            errorMessage = $_('signin.passwordverification')
            return false
        }
        if(password != rPassword){
            errorMessage = $_('signin.passwordmatch')
            return false
        }
        return true
    }

    function verify(){
        if(!verifyUsername()) {error = true; return}
        if(!verifyName()) {error = true; return}
        if(!verifyPassword()) {error = true; return}
        error = false
        verified = true
    }

    async function signin(){
        verify()
        if(!verified) return
        let user = new UserCreate();
        user.name = name
        user.password = password
        user.username = username
        ApiClient.createUser(user).then(user => {
            if (user.name == null){
                errorMessage = $_('signin.creationerror')
                error = true
            }
            else{
                message.set($_('signin.creationsuccess'))
                activeSection.set(Section.LOGIN)
            }
        })
    }
    
</script>

<div class="section">
    <div class="card">
        <p class="m-auto text-3xl mb-4">{ $_('general.signin') }</p>
        <div class="flex flex-col mb-4">
            <input type="text" placeholder="{$_('user.username')}" bind:value={username} class="text-input"/>
            <input type="text" placeholder="{ $_('user.name') }" bind:value={name} class="text-input"/>
            <input type="password" placeholder="{ $_('user.password') }" bind:value={password} class="text-input"/>
            <input type="password" placeholder="{ $_('user.rpassword') }" bind:value={rPassword} class="text-input"/>
        </div>
        <button on:click={signin} class="button">{ $_('general.signin') }</button>
        {#if error}
            <div class="mt-4">
                <AlertBox message={errorMessage}/>
            </div>
        {/if}
    </div>
</div>