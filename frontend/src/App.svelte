<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
    import MessageBox from "./lib/components/MessageBox.svelte";
  import { Section } from "./lib/Sections";
  import AboutSection from "./lib/sections/AboutSection.svelte";
  import AccountSection from "./lib/sections/AccountSection.svelte";
  import ClockInSection from "./lib/sections/ClockInSection.svelte";
  import ExportSection from "./lib/sections/ExportSection.svelte";
  import HistorySection from "./lib/sections/HistorySection.svelte";
  import LoginSection from "./lib/sections/LoginSection.svelte";
  import SigninSection from "./lib/sections/SigninSection.svelte";
  import Sidebar from "./lib/sidebar/Sidebar.svelte";
  import {activeSection, message} from "./store"
  import {initi18n} from "./i18n"
  import { isLoading } from 'svelte-i18n'

  initi18n()
</script>

<main>
  {#if $isLoading}
  <p></p>
  {:else}
    <div class="flex transition-all">
      <Sidebar/>
      {#if $activeSection == Section.CLOCKIN}
        <ClockInSection />
      {:else if $activeSection == Section.HISTORY}
        <HistorySection />
      {:else if $activeSection == Section.ACCOUNT}
        <AccountSection />
      {:else if $activeSection == Section.EXPORT}
        <ExportSection />
      {:else if $activeSection == Section.ABOUT}
        <AboutSection />
      {:else if $activeSection == Section.LOGIN}
        <LoginSection/>
      {:else if $activeSection == Section.SIGNIN}
        <SigninSection/>
      {/if}
    </div>
    {#if $message}
      <MessageBox/>
    {/if}
  {/if}
</main>

<style>
</style>
