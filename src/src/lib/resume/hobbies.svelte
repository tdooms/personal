<script lang="ts">
    const hobbies = [
        "cycling",
        "gaming",
        "programming",
        "music",
        "chess",
        "cooking"
    ]
</script>

<style>
.tag:not(body).is-white{ color: inherit !important; }
</style>


<h5 class="title is-5"> Hobbies </h5>
<div class="tags">
{#each hobbies as hobby}
    <div class="tag is-medium is-white"> <p>{hobby}</p> </div>
{/each}
</div>