<script lang="ts">
    export let title;
    export let date;
</script>

<div class="content has-text-justified" style="margin: 0 auto;max-width: 880px;">
    <!-- <h2 class="title is-2">{ title }</h2>
    <h6 class="subtitle is-6 mb-3">{ date }</h6> -->

    <div class="columns mb-0">
        <div class="column has-text-left">
            <h2 class="title is-2 mb-0">{ title }</h2>
        </div>
        <div class="column is-narrow has-text-right">
            <h6 class="title is-6 mt-5">{ date }</h6>
        </div>
    </div>

    <slot />
    <!-- <div class="buttons is-centered mt-5">
        <div class="button is-primary">Previous</div>
        <div class="button is-primary">Back</div>
        <div class="button is-primary">Next</div>
    </div> -->
    <div class="buttons is-centered mt-5">
        <a class="button is-primary" href="/blog">Back</a>
    </div>
    <div class="mb-6" />
</div>