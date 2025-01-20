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
        }
    ]

</script>

<div class="columns is-multiline">
    {#each blogs as blog}
        <div class="column is-4">
            <Card {...blog} />
        </div>
    {/each}
</div>
