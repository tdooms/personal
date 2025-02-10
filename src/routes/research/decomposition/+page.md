---
title: Bilinear Decomposition
date: 18 Jul 2024
kind: research
---

<script>
  import Resources from "$lib/research/resources.svelte";
  import Cite from "$lib/research/cite.svelte"
</script>

<p> Michael T. Pearce, <b>Thomas Dooms</b>, Alice Rigg </p>

<div class="mt-6"> </div>

<Resources 
    paper="https://arxiv.org/abs/2406.03947"
    code="https://github.com/tdooms/bilinear-decomposition"
    models="https://huggingface.co/collections/tdooms/bilinear-66991d82406db9529a7170d2"
/>

> This page has been superseded by a newer [version](/research/bilinear), which discusses our updated [paper](https://arxiv.org/pdf/2410.08417).

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
One prominent question that is often asked at this point is "if this is just doing linear operations, how can this be a good model component"?
While either side *is* linear, the whole is bilinear, which is non-linear to the input, which is all we need.

<div class="columns is-centered">
    <div class="column is-narrow">
        <figure>
        <img src="/research/bilinear/glu.svg" alt="GLU & Bilinear" />
        <figcaption>An ordinary GLU (left) and a bilinear layer (right). An ordinary GLU has a gate part which "selects" <br> which parts of the other side should be kept.
        In the case of a bilinear layer, this selection is instead continuous.</figcaption>
        </figure>
    </div>
</div>

As foreshadowed, the linearity of each branch is a really useful for interpretability. It allows to use techniques from linear algebra such as SVD and have them actually be meaningful. However, that's not all, each output of a bilinear layer can be described quite elegantly; it's a sum of (weighted) pairwise input feature interactions. Note that this is impossible to do for a layer with a ReLU (or other activation functions), there is no clean formula that describes how features interact.

### Decomposing The Weights

We can exploit all these facts to find useful direction for each output of such a layer.
While we won't go into details, these are the main ideaas behind the decomposition.

1. The feature interactions of a bilinear layer output can be written as $\textbf{x}A\textbf{x}$ where we call $A$ the interaction matrix. Each entry in the matrix is the weight of how strong the feature at the row and column should interact. Due to this structure, it is a symmetric matrix. While this is nice, this interaction matrix is hard to visualize for any non-trivial problem (especially if the features are structured, such as images).

2. Luckily, we can leverage this symmetric property and perform an eigendecomposition on these matrices. This operation decomposes the matrix into $\sum_i \lambda_i v_i \otimes v_i$ (where $\otimes$ is an outer product) where values of $\lambda$ decrease. In words, this finds which vectors best describe the interaction matrix. We can then easily visualize these vectors.

3. Since a sum of symmetric matrices is still symmetric, this can be done for any output direction not just single outputs. For instance, we can take the difference between two features by subtracting the interaction matrices.

The following images depict the most important eigenvector for a set of digits in a single-layer MNIST model.

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

This shows that exploiting these properties of the bilinear layer can yield very interpretable structure from the weights alone!

### Language Models

We can use the same technique of replacing MLPs in a transformer with its bilinear variant.
This allows us to understand the computation going on in MLPs on a deep level.

The following figure specifically extends the technique to attention, we derive the most important features for swim and then examine which samples activate them most strongly.
We see that the first feature fires on sea related contexts while the second is mostly grammatical.

<figure>
    <img src="/research/bilinear/swim-cropped.svg" alt="Eigenvectors of swim" width="880" />
    <figcaption></figcaption>
</figure>

### Future Work

Currently, this approach is only feasible on shallow models as the number of eigenvectors grows exponentially with layer size.
We are exploring techniques to reduce this.

This work mostly aimed to show the interpretability instead of leveraging the decomposition towards concrete findings. 
One could imagine using this to uncover induction behaviour in language models or curve detector circuits in vision models in a principled manner.

<Cite text="@misc&#123;pearce2024weightbaseddecompositioncasebilinear,
        title=&#123;Weight-based Decomposition: A Case for Bilinear MLPs&#125;,
        author=&#123;Michael T. Pearce and Thomas Dooms and Alice Rigg&#125;,
        year=&#123;2024&#125;,
        eprint=&#123;2406.03947&#125;,
        archivePrefix=&#123;arXiv&#125;,
        primaryClass=&#123;cs.LG&#125;,
        url=&#123;https://arxiv.org/abs/2406.03947&#125;,
    &#125;">
</Cite>