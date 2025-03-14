---
title: Introducing X-nets
date: 12 Mar 2024
kind: research
---

<script>
  import Resources from "$lib/research/resources.svelte";
  import Cite from "$lib/research/cite.svelte"
</script>

<p> <b>Thomas Dooms*</b>, Ward Gauderis*, Geraint Wiggins, Jose M. Oramas </p>

<div class="mt-6"> </div>

<Resources
    paper="https://openreview.net/pdf?id=bXAt5iZ69l"
    video="https://www.youtube.com/watch?v=yUGZVPJlvzY"
/>

### Overview

Previous work in weight-based interpretability was limited to single layer analysis.
To interpret deeper models, sparse dictionary learning was necessary divide into subproblems.
Ideally, we would want to extract features, purely from the weights, even in deeper models.

This paper introduces a theoretic framework toward scalable weight-based interpretability.
These insights are used toward a new algorithm that globally decomposes the weights of a model, akin to SVD.
While this algorithm optimises toward rank, it naturally finds sparse structure, which is easy to understand.

This decomposition can be used to find the globally most important eigenvectors for each output class.
we study a multi-layer image model trained on the SVHN dataset (a more challenging variant of MNIST that single layer models can't solve).

<figure>
    <img src="/research/xnet/digits.png" alt="Top SVHN eigenvectors." width=500px />
    <figcaption>Top eigenvectors (extracted from the weights) per class of a 3-layer SVHN classifier.</figcaption>
</figure>

The decomposition can also be used to morph the model into a sparse compositional tree.
The bottom row of this figure shows the most important 'features' to the model.
These reveal important strokes or proto-digits, hinting the model has learned sensible structure.
The next layers represent 'interaction matrices', which describe how features interact.
The first two layers only combine a handful of features together.
The last layer densely (but still low-rank) combines these composed features into a prediction.
Tracing out paths through this tree shows how the model forms specific predictions.

<figure>
    <img src="/research/xnet/cores.png" alt="Sparse weights of the same model." width=500px />
    <figcaption>Visualisations of the weights after a global decomposition.</figcaption>
</figure>

<Cite text="@inproceedings&#123;
    dooms2025compositionality,
    title=&#123;Compositionality Unlocks Deep Interpretable Models&#125;,
    author=&#123;Thomas Dooms and Ward Gauderis and Geraint Wiggins and Jose Oramas&#125;,
    booktitle=&#123;Submitted to AAAI'25 workshop on CoLoRAI - Connecting Low-Rank Representations in AI&#125;,
    year=&#123;2025&#125;,
    url=&#123;https://openreview.net/forum?id=bXAt5iZ69l&#125;,
    note=&#123;under review&#125;
&#125;" />
