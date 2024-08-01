<script lang="ts">
    import { copy } from 'svelte-copy';

    export let text: string;
</script>

<div class="card">
    <header class="card-header has-background-grey-light">
        <p class="card-header-title mb-0">How to cite</p>
        <button class="card-header-icon" aria-label="more options">
            <span class="icon">
                <i class="fa-solid fa-copy" aria-hidden="true" use:copy={text.replace("&#125;", '}').replace("&#123;", '{')}></i>
            </span>
        </button>
    </header>
    <small>
    <pre class="card-content" id="cite-slot">{text}</pre>
    </small>
</div>