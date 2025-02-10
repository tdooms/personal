<script lang="ts">
    export let paper: string;
    export let code: string;
    export let models: string;
    export let demo: string;
</script>

<div class="columns is-centered">

    {#if paper}
    <div class="column is-2 px-3">
        <a class="box has-text-centered" href={paper} target="_blank">
            <span class="icon is-large">
                <i class="fa-regular fa-xl fa-file"></i>
            </span>
            <p> Paper </p>
        </a>
    </div>
    {/if}

    {#if code}
    <div class="column is-2">
        <a class="box has-text-centered" href={code} target="_blank">
            <span class="icon is-large">
                <i class="fa-solid fa-xl fa-code"></i>
            </span>
            <p> Code </p>
        </a>
    </div>
    {/if}

    {#if models}
    <div class="column is-2">
        <a class="box has-text-centered" href={models} target="_blank">
            <span class="icon is-large">
                <i class="fa-solid fa-xl fa-database"></i>
            </span>
            <p> Models </p>
        </a>
    </div>
    {/if}

    {#if demo}
    <div class="column is-2 px-3">
        <a class="box has-text-centered" href={demo}>
            <span class="icon is-large">
                <i class="fa-solid fa-xl fa-flask"></i>
            </span>
            <p> Demo </p>
        </a>
    </div>
    {/if}

</div>