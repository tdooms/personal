<script lang="ts">
    import Box from "$lib/box.svelte";
    import Presentation from "$lib/presentation.svelte";

const papers = [
    {
        "image": "research/bilinear.png",
        "title": "Weight-based decomposition",
        "alt": "A magnifying glass",
        "date": "Oct 2024",
        "published": "",
        "description": "Using bilinear MLPs to reverse-engineer image and language models from their weights.",
        "link": "research/bilinear"
    },
    {
        "image": "research/bilinear.png",
        "title": "Bilinear MLPs",
        "alt": "A magnifying glass",
        "date": "Jun 2024",
        "published": "ICML MI workshop",
        "description": "Introducing bilinear MLPs as a new approach to weight-based interpretability.",
        "link": "research/bilinear"
    },
    {
        "image": "research/disentangle.png",
        "title": "Tokenized SAEs",
        "alt": "A split arrow",
        "date": "Jun 2024",
        "published": "ICML MI workshop",
        "description": "We propose using a per-token bias in SAEs to separate token reconstructions from interesting, semantic features.",
        "link": "research/tokenized"
    },
    {
        "image": "research/trifecta.png",
        "title": "The Trifecta",
        "alt": "A trident",
        "date": "Nov 2023",
        "published": "TMLR",
        "description": "Three techniques to significantly improve the Forward-Forward algorithm. We achieve 84% on CIFAR-10.",
        "link": "research/trifecta"
    }
]
const presentations = [
    {
        "title": "Weight-based interpretability",
        "date": "Jan 2024",
        "icon": "fa-diagram-next",
        "published": "Flanders AI Day",
        "description": "A quick primer on weight-based interpretability.",
        "link": "https://docs.google.com/presentation/d/1PdcHm0bgwIu3yhy6oWkwL1-fDGTPzTiyUilydognLlw/edit?usp=sharing",
    },
    {
        "title": "Introduction to Transformers",
        "date": "Jan 2024",
        "icon": "fa-diagram-next",
        "published": "Lab meeting",
        "description": "An informal introduction to the Transformer architecture.",
        "link": "https://docs.google.com/presentation/d/1VBMEUE1bb5h48DFhSBZ8L20PJ1LwN1LMippYOZjLHos/edit?usp=sharing",
    },
    {
        "title": "Embodied Language Models",
        "date": "Jun 2023",
        "icon": "fa-robot",
        "published": "University Lecture",
        "description": "A presentation on the integration of LLMs into robotics.",
        "link": "https://docs.google.com/presentation/d/1oURN_cmpF-GYLYVCDLU7teZ2RL_I5oeMH8onfU7ffNM/edit?usp=sharing",
    },
    {
        "title": "Diffusion Models",
        "icon": "fa-images",
        "date": "Jan 2023",
        "published": "University Lecture",
        "description": "A brief overview of diffusion models and the current SOTA.",
        "link": "https://docs.google.com/presentation/d/1s0gPSXldiHgu4AYji_ELhoDTNNluNiKgq8Ry_t3cJM0/edit?usp=sharing",
    },
]
const posts = [
    {
        "title": "Tokenized SAEs",
        "date": "Aug 2024",
        "icon": "fa-diagram-next",
        "published": "Lesswrong",
        "description": "An informal write-up of the Tokenized SAEs paper.",
        "link": "https://www.lesswrong.com/posts/P8qLZco6Zq8LaLHe9/tokenized-saes-infusing-per-token-biases",
    },
]

</script>


<h5 class="title is-5"> Papers </h5>
{#each papers as paper}
    <Box {...paper} />
{/each}

<h5 class="title is-5 mt-6"> Presentations </h5>
{#each presentations as presentation}
    <Box {...presentation} />
{/each}

<h5 class="title is-5 mt-6"> Posts </h5>
{#each posts as post}
    <Box {...post} />
{/each}