<script lang="ts">
    import Language from "$lib/language.svelte";

    const languages = [
        {
            "name": "Dutch",
            "level": "Native",
            "percent": 100
        },
        {
            "name": "English",
            "level": "Strong Proficiency",
            "percent": 90
        },
        {
            "name": "French",
            "level": "Bilingual",
            "percent": 80
        },
        {
            "name": "Spanish",
            "level": "Basic",
            "percent": 40
        },
        {
            "name": "German",
            "level": "Basic",
            "percent": 30
        }
    ]
</script>

<h5 class="title is-5"> Languages </h5>
<div class="box" style="height:294px">
    {#each languages as language}
        <Language {...language} />
    {/each}
</div>