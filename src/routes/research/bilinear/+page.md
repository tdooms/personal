---
title: Bilinear Decomposition
date: 18 Jul 2024
kind: research
---

<script>
  import Resources from "$lib/resources.svelte";
  import Cite from "$lib/cite.svelte"
</script>

<p> Michael T. Pearce, <b>Thomas Dooms</b>, Alice Rigg </p>

<div class="mt-6"> </div>
<Resources paper="https://arxiv.org/abs/2406.03947" code="https://github.com/tdooms/bilinear-decomposition" models="https://huggingface.co/collections/tdooms/bilinear-66991d82406db9529a7170d2" />

### Background

Understanding models from their weights has long been a pipedream of many interpretability researchers.
The main obstacle to this is that ReLUs (or non-linear activation functions in general) are not meaningful without inputs.
Since, any small change in input can cause a ReLU to "activate".
The *only* way to compute the output of a ReLU is to actually provide it an input and just see what happens.
This has made it notoriously hard to put any form of guarantees on model behaviour and has led to a plethora of input-dependent research in the past.
The one thing giving rise to the astounding capabilities is also the thing that makes them *really* hard to study.

Bilinear layers, may be able to offer us the best of both worlds; weights which we can meaningfully study while retaining the current capabilities.
Bilinear layers are part of the gated linear unit (GLU) family, which are gaining lots of traction due to their accuracy benefits recently.
The bilinear layer is the simplest form of a GLU, using no activation function at all.

### Decomposing The Weights

By omitting the activation function, we retain a powerful close-to-linear structure.
This can be analyzed using well-known techniques from linear algebra.
Specifically, since our weights are symmetric, we use the eigendecomposition.

<!-- Doesn't seem like I can easily convert this to pure markdown sadly -->
<div class="columns">
    <div class="column">
        <img src="/research/bilinear/digit_0.svg" alt="digit 0" />
    </div>
    <div class="column">
        <img src="/research/bilinear/digit_1.svg" alt="digit 1" />
    </div>
    <div class="column">
        <img src="/research/bilinear/digit_5.svg" alt="digit 5" />
    </div>
    <div class="column">
        <img src="/research/bilinear/digit_6.svg" alt="digit 6" />
    </div>
</div>


The images above show the most important eigenvectors for the shown output classes.
This shows that the eigendecomposition can reveal meaningful structure in the model.

### Language Models

We can use the same technique of replacing MLPs in a transformer with its bilinear variant.
This allows us to understand the computation going on in MLPs on a deep level.

### Future Work

Currently, this approach is only feasible on shallow models as the number of eigenvectors grows exponentially with layer size.
We are exploring techniques to reduce this.

<Cite>
    @misc&#123;pearce2024weightbaseddecompositioncasebilinear,
        title=&#123;Weight-based Decomposition: A Case for Bilinear MLPs&#125;,
        author=&#123;Michael T. Pearce and Thomas Dooms and Alice Rigg&#125;,
        year=&#123;2024&#125;,
        eprint=&#123;2406.03947&#125;,
        archivePrefix=&#123;arXiv&#125;,
        primaryClass=&#123;cs.LG&#125;,
        url=&#123;https://arxiv.org/abs/2406.03947&#125;,
    &#125;
</Cite>
