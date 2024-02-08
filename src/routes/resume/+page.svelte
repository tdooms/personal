<script>
    import Education from "$lib/resume/education.svelte";
    import Languages from "$lib/resume/languages.svelte";
    import Programming from "$lib/resume/programming.svelte";
    import Work from "$lib/resume/work.svelte";
    import Frameworks from "$lib/resume/frameworks.svelte";
    import Awards from "$lib/resume/awards.svelte";
    import Traits from "$lib/resume/traits.svelte";
    import Courses from "$lib/resume/courses.svelte";
    import Hobbies from "$lib/resume/hobbies.svelte";
</script>

<div class="columns">
    <div class="column is-4">
        <Education />
    </div>
    <div class="column">
        <Work />
    </div>
</div>

<div class="columns">
    <div class="column is-3">
        <Languages />
        <Hobbies />
        <!-- <Courses /> -->
    </div>
    <div class="column is-3">
        <Programming />
        <Traits />
    </div>
    <div class="column is-6">
        <Frameworks />
        <Awards />
    </div>
</div>
