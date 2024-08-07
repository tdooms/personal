<script lang="ts">
    import Paper from "$lib/paper.svelte";
    import Presentation from "$lib/presentation.svelte";

const papers = [
    {
        "image": "research/bilinear.png",
        "title": "Bilinear Decomposition",
        "alt": "A magnifying glass",
        "date": "Jun 2024",
        "description": "Using bilinear MLPs to reverse-engineer shallow MNIST and Tiny Stories models using only their weights.",
        "paper": "https://arxiv.org/abs/2406.03947",
        "code": "https://github.com/tdooms/bilinear-decomposition",
        "link": "research/bilinear"
    },
    {
        "image": "research/disentangle.png",
        "title": "Tokenized SAEs",
        "alt": "A split arrow",
        "date": "Jun 2024",
        "description": "We propose using a per-token bias in SAEs to separate token reconstructions from interesting, semantic features.",
        "paper": "documents/tokenized_saes.pdf",
        "code": "",
        "link": "research/tokenized"
    },
    {
        "image": "research/trifecta.png",
        "title": "The Trifecta",
        "alt": "A trident",
        "date": "Nov 2023",
        "description": "Three techniques to significantly improve the Forward-Forward algorithm. We achieve 84% on CIFAR-10.",
        "paper": "https://arxiv.org/abs/2311.18130",
        "code": "https://github.com/tdooms/trifecta",
        "link": "research/trifecta"
    }
]
const presentations = [
    {
        "title": "Introduction to Transformers",
        "date": "Jan 2024",
        "icon": "fa-diagram-next",
        "description": "An informal introduction to the Transformer architecture.",
        "slides": "https://docs.google.com/presentation/d/1VBMEUE1bb5h48DFhSBZ8L20PJ1LwN1LMippYOZjLHos/edit?usp=sharing",
    },
    {
        "title": "Embodied Language Models",
        "date": "Jun 2023",
        "icon": "fa-robot",
        "description": "A presentation on the integration of LLMs into robotics.",
        "slides": "https://docs.google.com/presentation/d/1oURN_cmpF-GYLYVCDLU7teZ2RL_I5oeMH8onfU7ffNM/edit?usp=sharing",
    },
    {
        "title": "Diffusion Models",
        "icon": "fa-images",
        "date": "Jan 2023",
        "description": "A brief overview of diffusion models and the current SOTA.",
        "slides": "https://docs.google.com/presentation/d/1s0gPSXldiHgu4AYji_ELhoDTNNluNiKgq8Ry_t3cJM0/edit?usp=sharing",
    },
]

</script>


<div class="columns">
    <div class="column is-6">
        <h5 class="title is-5"> Papers </h5>
        {#each papers as paper}
            <Paper {...paper} />
        {/each}
        <p> And much more to come...</p>
    </div>

    <div class="column is-6">
        <h5 class="title is-5"> Presentations </h5>
        {#each presentations as presentation}
            <Presentation {...presentation} />
        {/each}
    </div>
</div>