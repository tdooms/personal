<script lang="ts">
    import Media from "$lib/media.svelte";

    const education = [
        {
            "date": "Sep 2021 - Jun 2023",
            "title": "Master in Artificial Intelligence",
            "grade": "Summa cum laude (87%)"
        },
        {
            "date": "Sep 2019 - Aug 2021",
            "title": "Honours Programme in Science",
            "grade": "Summa cum laude (85%)"
        },
        {
            "date": "Sep 2018 - Jun 2021",
            "title": "Bachelor of Computer Science",
            "grade": "Magna cum laude (83%)"
        }
    ]
</script>


<h5 class="title is-5"> Education </h5>
<Media icon="icons/ua.png" date="Nov 2023 - TBD" title="PhD in Machine Learning">
    <p class="mt-2"> <b> Subject: </b>Weight-based interpretability</p>
    <p> <b> Advisor: </b> Jose M. Oramas </p>
</Media>

{#each education as study}
    <Media icon="icons/ua.png" date={study.date} title={study.title}>
        <p class="mt-2"> <b> Grade: </b> {study.grade} </p>
    </Media>
{/each}