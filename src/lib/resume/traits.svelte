<script lang="ts">
        const traits = [
        "eager to learn",
        "creative",
        "quick-study",
        "detail-oriented"
    ]
</script>

<style>
    .tag:not(body).is-white{ color: inherit !important; }
</style>


<h5 class="title is-5"> Traits </h5>
<div class="tags">
    {#each traits as trait}
        <div class="tag is-medium is-white"> <p>{trait}</p> </div>
    {/each}
</div>