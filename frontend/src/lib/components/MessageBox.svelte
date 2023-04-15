<script lang="ts">
    import { message } from "../../store"

    let timer = setTimeout(close, 5000)

    function close(){
        clearTimeout(timer)
        message.set("")
    }
</script>

<div class="message-box">
    <div class="flex flex-row">
        <p class="mr-2">{$message}</p>
        <button class="tiny-btn mx-1" on:click={close}>×</button>
    </div>
</div>