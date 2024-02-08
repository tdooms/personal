<script lang="ts">
    import Media from "$lib/media.svelte";

    const work = [
        {
            "icon": "icons/cuberdon.jpeg",
            "date": "Jul 2022 - Sep 2022",
            "title": "Cuberdon Software Solutions",
            "text": "Involved with the groundwork for a next-generation customer portal with professional dashboards, dynamic news and document organisation. Full-stack development with Vue, ASP.NET and SQL."
        },
        {
            "icon": "icons/keysight.png",
            "date": "Sep 2021 - Jun 2022",
            "title": "Keysight Technologies",
            "text": "Research project to analyse and predict the power consumption of embedded devices."
        },
        {
            "date": "Mar 2021 - Sep 2021",
            "icon": "icons/uza.png",
            "title": "Antwerp University Hospital",
            "text": "Internship linked to honours programme. Data measurment and analysis of food temperature for premature babies."
        },
        {
            "icon": "icons/cuberdon.jpeg",
            "date": "Jul 2022 - Sep 2022",
            "title": "Cuberdon Software Solutions",
            "text": "Responsible for integrating a form analyser and speech recognition into a website."
        }
    ]
</script>

<h5 class="title is-5"> Work Experience </h5>
{#each work as job}
    <Media {...job}> <p class="mt-2"> {job.text} </p> </Media>
{/each}