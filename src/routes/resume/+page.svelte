<script lang="ts">
    import Education from "$lib/resume/education.svelte";
    import Languages from "$lib/resume/languages.svelte";
    import Work from "$lib/resume/work.svelte";
    import Awards from "$lib/resume/awards.svelte";
</script>

<div class="columns">
    <div class="column is-5">
        <h5 class="title ml-2 is-5"> Education </h5>
        <Education />
    </div>
    <div class="column is-7">
        <h5 class="title ml-2 is-5"> Work Experience </h5>
        <Work />
    </div>
</div>

<div class="columns">
    <div class="column is-4">
        <h5 class="title ml-2 is-5"> Languages </h5>
        <Languages />
    </div>
    <div class="column is-8">
        <h5 class="title ml-2 is-5"> Awards </h5>
        <Awards />
    </div>
</div>
