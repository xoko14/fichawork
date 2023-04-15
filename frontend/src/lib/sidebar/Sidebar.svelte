<script lang="ts">
import { Section } from "../Sections";
import SidebarIcon from "./SidebarIcon.svelte";
import { activeSection, token } from "./../../store";
import {_} from "svelte-i18n"

</script>

<div class="sidebar">
    {#if $token}
        <SidebarIcon icon="punch_clock" tooltip="{$_('general.clockin')}" on:click={() => activeSection.set(Section.CLOCKIN)}/>
        <SidebarIcon icon="history" tooltip="{$_('general.history')}" on:click={() => activeSection.set(Section.HISTORY)}/>
        <SidebarIcon icon="account_circle" tooltip="{$_('general.account')}" on:click={() => activeSection.set(Section.ACCOUNT)}/>
        <SidebarIcon icon="cloud_download" tooltip="{$_('general.export')}" on:click={() => activeSection.set(Section.EXPORT)}/>
        <SidebarIcon icon="info" tooltip="{$_('general.about')}" on:click={() => activeSection.set(Section.ABOUT)}/>
    {:else}
        <SidebarIcon icon="login" tooltip="{$_('general.login')}" on:click={() => activeSection.set(Section.LOGIN)}/>
        <SidebarIcon icon="person_add" tooltip="{$_('general.signin')}" on:click={() => activeSection.set(Section.SIGNIN)}/>
        <SidebarIcon icon="info" tooltip="{$_('general.about')}" on:click={() => activeSection.set(Section.ABOUT)}/>
    {/if}
</div>