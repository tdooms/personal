<script lang="ts">
    import Card from "$lib/card.svelte";

    const blogs = [

        {
            title: "An analogy of neural networks and the human body",
            date: "17 Jan 2024",
            image: "blogs/medicine.png",
            location: "blog/medicine"
        },

        {
            title: "Reflecting on beginner questions in machine learning",
            date: "11 Jan 2024",
            image: "blogs/questions.png",
            location: "blog/questions"
        },
        {
            title: "My take on mechanistic interpretability techniques",
            date: "coming soon",
            image: "blogs/tools.png",
            location: "blog/tools"
        },
        {
            title: "Some thoughts on alternative deep learning paradigms",
            date: "coming soon",
            image: "blogs/alternatives.png",
            location: "blog/alternatives"
        },
    ]

</script>

<div class="columns is-multiline mx-6">
    {#each blogs as blog}
        <div class="column is-4">
            <Card {...blog} />
        </div>
    {/each}
</div>
