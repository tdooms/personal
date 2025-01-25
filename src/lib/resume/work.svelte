<script lang="ts">
    import Media from "$lib/media.svelte";

    const work = [
        {
            "icon": "icons/mats.jpg",
            "date": "May 2024 - Sep 2024",
            "title": "MATS Program",
            "text": "Mechanistic interpretability stream with Neel Nanda and Lee Sharkey. Wrote an ICLR paper and two ICML workshop papers."
        },
        {
            "icon": "icons/cuberdon.jpeg",
            "date": "Jul 2022 - Sep 2022",
            "title": "Cuberdon Software Solutions",
            "text": "Full-stack development with Vue and ASP.NET for a customer portal."
        },
        {
            "icon": "icons/keysight.png",
            "date": "Sep 2021 - Jun 2022",
            "title": "Keysight Technologies",
            "text": "Research to predict the power consumption of embedded devices."
        },
        {
            "date": "Mar 2021 - Sep 2021",
            "icon": "icons/uza.png",
            "title": "Antwerp University Hospital",
            "text": "Data analysis of food temperature for premature babies."
        },
        // {
        //     "icon": "icons/cuberdon.jpeg",
        //     "date": "Jul 2022 - Sep 2022",
        //     "title": "Cuberdon Software Solutions",
        //     "text": "Responsible for integrating a form analyser and speech recognition into a website."
        // }
    ]
</script>


{#each work as job}
    <Media {...job}> <p class="mt-2"> {job.text} </p> </Media>
{/each}