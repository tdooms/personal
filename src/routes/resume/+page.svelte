<script lang="ts">
    import Education from "$lib/resume/education.svelte";
    import Languages from "$lib/resume/languages.svelte";
    import Work from "$lib/resume/work.svelte";
    import Awards from "$lib/resume/awards.svelte";
</script>

<div class="columns">
    <div class="column is-5">
        <Education />
    </div>
    <div class="column is-7">
        <Work />
    </div>
</div>

<div class="columns">
    <div class="column is-4">
        <Languages />
    </div>
    <div class="column is-8">
        <Awards />
    </div>
</div>
