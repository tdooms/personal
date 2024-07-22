---
title: Tokenized SAEs
date: 18 Jul 2024
kind: research
---

<script>
  import Resources from "$lib/resources.svelte";
  import Cite from "$lib/cite.svelte"
</script>

<p> <b>Thomas Dooms</b>, Daniel Wilhelm </p>

<div class="mt-6"> </div>
<Resources paper="https://openreview.net/attachment?id=5Eas7HCe38&name=pdf" />

### Background

Sparse auto-encoders have become the main focus of mechinterp research; they provide a means to extract interpretable bases from intermediate stages of the model.
They achieve this by locally reconstructing activations. While this has been remarkably effective, this has some suboptimal side-effects, in this paper we focus on distribution-dependent features.
We explain why these arise and propose a technique to separate some of them from the main reconstruction.

### Motivation


### Results

We show adding a lookup table improves the final reconstructions by a significant margin on GPT-2 layer 8.

![Pareto curve of the normalized mean squared error](/research/tokenized/pareto_nmse.svg)

Beyond this, by forcing the SAE to use directions we know to be useful, it is able to learn much more quickly.
We measure how much faster TSAEs reach the final reconstruction value of their baseline variant.

![Speedup of TSAE vs baseline](/research/tokenized/speedup_gpt2.svg)

This huge speedup results in TSAEs reaching really high fidelity reconstructions in only a few minutes across GPT-2 layers.
We believe this will be really useful for quickly iterating on SAE suites.

Common intuition is that this lookup table would become less effective with depth or model complexity.
However, our results show that Tokenized SAEs remain effective on Pythia 1.4B, even at layer 20.
We again show the speedup.

![Speedup of TSAE vs baseline](/research/tokenized/speedup_pythia.svg)

### Future Work
