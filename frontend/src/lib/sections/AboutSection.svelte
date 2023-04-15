<script lang="ts">
    import {_} from 'svelte-i18n'
</script>

<div class="section">
    <div class="card">
        <div class="card-section">
            <p class="text-3xl">FichaWork</p>
            <br/>
            <p>{$_('about.desc')}</p>
            <p>
                {$_('about.repodesc')} <a href="https://gitlab.shoudev.com/shou/ficha-work" class="link">{$_('about.repo')}</a>.
            </p>
            <p>{$_('about.madeby')} <a href="https://shoudev.com" class="link">shoudev</a>.</p>
        </div>
    </div>
</div>
