---
title: Tokenized SAEs
date: 18 Jul 2024
kind: research
---

<script>
    import Resources from "$lib/research/resources.svelte";
    import Cite from "$lib/research/cite.svelte";
    import Pareto from "$lib/research/pareto.svelte"
</script>

<p> <b>Thomas Dooms</b>, Daniel Wilhelm </p>

<div class="mt-6"> </div>
<Resources paper="https://openreview.net/attachment?id=5Eas7HCe38&name=pdf" code="https://github.com/tdooms/smol-sae" />

### Background

Sparse auto-encoders have become the main focus of mechinterp research; they provide a means to extract interpretable bases from intermediate stages of the model.
They achieve this by locally reconstructing activations. While this has been remarkably effective, this has some suboptimal side-effects, in this paper we focus on distribution-dependent features.
We explain why these arise and propose a technique to separate some of them from the main reconstruction.

### Motivation

The motivation for this work can be framed from multiple perspectives. Initially, it arose form spending quite a bit of time on Neuronpedia and seeing that the vast majority of features is token-based instead of context-based. From a training perspective, this makes sense due to the following two facts:

- Token directions are generally the most important direction in the representation.
- Token directions are more frequent than some specific context-based representation.

Regardless of the reason they exist, ideally, they wouldn't clutter the (generally limited) learned features.

### Method

I generally introduce the proposed method as a simple trick to remove these single-token features which seems to work surprisingly well.
The main idea is to introduce a lookup table to the decoder that is indexed by the original token of the current activations.
This table then takes care of the "base" reconstruction for each token.
We denote the hidden activations that originate from some token $t$ as $x_t$ and the sparsity/activation function as $\sigma$.

$$a_t = \sigma(W_{enc}(x_t - b_{dec}))$$

$$\hat{x}_t = W_{dec}(a_t) + b_{dec} + \mathbf{W_{lookup}(t)}$$

There are some slight caveats to training this lookup table, which are described in the paper. Outside of this, it's really just that simple.

### Results

We show adding a lookup table improves the final reconstructions by a significant margin on GPT-2 layer 8.

<Pareto />

Beyond this, by forcing the SAE to use directions we know to be useful, it is able to learn much more quickly.
We measure how much faster TSAEs reach the final reconstruction value of their baseline variant.

<figure>
    <img src="/research/tokenized/speedup_gpt2.svg" alt="Speedup of TSAE vs baseline" />
    <figcaption>Speedup of TSAE vs baseline on GPT-2 small</figcaption>
</figure>

This huge speedup results in TSAEs reaching really high fidelity reconstructions in only a few minutes across GPT-2 layers.
We believe this will be really useful for quickly iterating on SAE suites.

Common intuition is that this lookup table would become less effective with depth or model complexity.
However, our results show that Tokenized SAEs remain effective on Pythia 1.4B, even at layer 20.
We again show the speedup.

<figure>
    <img src="/research/tokenized/speedup_pythia.svg" alt="Speedup of TSAE vs baseline" />
    <figcaption>Speedup of TSAE vs baseline on Pythia 1.4B</figcaption>
</figure>

### Future Work

The general idea of incorporating inductive bias into SAEs seems interesting to pursue. Trivially, a more general (sparse) n-gram lookup table could be used. Furthermore, features from a preceding SAE could similarly be used into some kind of lookup table.
