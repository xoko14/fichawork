<script lang="ts">
    import { _ } from "svelte-i18n"
    import { currentUser, token, activeSection} from "./../../store";

    import { onMount } from "svelte";

    import { ApiClient } from "./../../api/apiClient";
    import type { User } from "./../../api/models/User";
    import { Section } from "../Sections";

    let strtoken: string;

    token.subscribe((val) => {strtoken = val})

    //onMount(async () =>
        new ApiClient(strtoken)
            .getCurrentUser()
            .then((user) => currentUser.set(user))
    //);

    function logout(){
        token.set("")
        activeSection.set(Section.LOGIN)
    }

</script>

<div class="section">
    <div class="card">
        <p class="m-auto text-3xl mb-4">{$_("account.greeting", { values: { name: $currentUser.name }})}</p>
        <button on:click={logout} class="button">{ $_('account.logout') }</button>
    </div>
</div>
