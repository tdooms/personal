<script lang="ts">
    import Box from "$lib/box.svelte";

const papers = [
    {
        "title": "Bilinear MLPs enable weight-based mechanistic interpretability",
        "date": "Oct 2024",
        "published": "ICLR'25",
        "description": "Using bilinear MLPs to reverse-engineer image and language models from their weights.",
        "link": "research/bilinear",
    },
    {
        "title": "Weight-based Decomposition: A Case for Bilinear MLPs",
        "date": "Jun 2024",
        "published": "ICML'24 MI workshop",
        "description": "Introducing bilinear MLPs as a new approach to weight-based interpretability.",
        "link": "research/decomposition"
    },
    {
        "title": "Tokenized SAEs: Disentangling SAE Reconstructions",
        "date": "Jun 2024",
        "published": "ICML'24 MI workshop",
        "description": "We propose using a per-token bias in SAEs to separate token reconstructions from interesting, semantic features.",
        "link": "research/tokenized"
    },
    {
        "title": "The Trifecta: Three techniques for deeper Forward-Forward networks",
        "date": "Nov 2023",
        "published": "TMLR",
        "description": "Three techniques to significantly improve the Forward-Forward algorithm. We achieve 84% on CIFAR-10.",
        "link": "research/trifecta"
    }
]
const posts = [
    {
        "title": "Tokenized SAEs",
        "date": "Aug 2024",
        "published": "Lesswrong",
        "description": "An informal write-up of the Tokenized SAEs paper.",
        "link": "https://www.lesswrong.com/posts/P8qLZco6Zq8LaLHe9/tokenized-saes-infusing-per-token-biases",
        "external": true
    },
]
const presentations = [
    {
        "title": "Weight-based interpretability",
        "date": "Oct 2024",
        "published": "Flanders AI Day",
        "description": "A quick primer on weight-based interpretability.",
        "link": "https://docs.google.com/presentation/d/1PdcHm0bgwIu3yhy6oWkwL1-fDGTPzTiyUilydognLlw/edit?usp=sharing",
        "external": true
    },
    {
        "title": "Introduction to Transformers",
        "date": "Jan 2024",
        "published": "Lab meeting",
        "description": "An informal introduction to the Transformer architecture.",
        "link": "https://docs.google.com/presentation/d/1VBMEUE1bb5h48DFhSBZ8L20PJ1LwN1LMippYOZjLHos/edit?usp=sharing",
        "external": true
    },
    {
        "title": "Embodied Language Models",
        "date": "Jun 2023",
        "published": "University lecture",
        "description": "A presentation on the integration of LLMs into robotics.",
        "link": "https://docs.google.com/presentation/d/1oURN_cmpF-GYLYVCDLU7teZ2RL_I5oeMH8onfU7ffNM/edit?usp=sharing",
        "external": true
    },
    {
        "title": "Diffusion Models",
        "date": "Jan 2023",
        "published": "University lecture",
        "description": "A brief overview of diffusion models and the current SOTA.",
        "link": "https://docs.google.com/presentation/d/1s0gPSXldiHgu4AYji_ELhoDTNNluNiKgq8Ry_t3cJM0/edit?usp=sharing",
        "external": true
    },
]


</script>


<h5 class="title is-5 ml-2 mt-1"> Papers </h5>
{#each papers as paper}
    <Box {...paper} />
{/each}

<h5 class="title is-5 ml-2 mt-6"> Posts </h5>
{#each posts as post}
    <Box {...post} />
{/each}

<h5 class="title is-5 ml-2 mt-6"> Presentations </h5>
{#each presentations as presentation}
    <Box {...presentation} />
{/each}


<div class="mb-6"></div>