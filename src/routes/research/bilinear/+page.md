---
title: Bilinear Decomposition
date: 15 Oct 2024
kind: research
---

<script>
  import Resources from "$lib/research/resources.svelte";
  import Cite from "$lib/research/cite.svelte"
</script>

<p> Michael T. Pearce*, <b>Thomas Dooms*</b>, Alice Rigg, Jose M. Oramas, Lee Sharkey </p>

<div class="mt-6"> </div>
<Resources paper="https://arxiv.org/pdf/2410.08417" code="https://github.com/tdooms/bilinear-decomposition" models="https://huggingface.co/collections/tdooms/bilinear-transformers-tinystories-670e352e0f552bab121d861f" />

### Background

Understanding models from their weights has long been a pipedream of many interpretability researchers.
The main obstacle to this is that ReLUs (or non-linear activation functions in general) obscure the relation between inputs,
outputs, and weights. Since any slight change in input can cause a ReLU to activate. The only way to compute a ReLU's output
is to provide an input and see what happens. Consequently, input features can interact in convoluted ways,
making it notoriously hard to guarantee model behavior, and this has led to a plethora of input-dependent research in the past.
The one thing that gives rise to astounding capabilities is also what makes them hard to study.

Bilinear layers are a promising alternative to ordinary layers, offering the best of both worlds: high accuracy and meaningful weights.
They are part of the gated linear unit (GLU) family, which has recently gained traction due to their accuracy benefits.
The bilinear layer is the simplest form of a GLU, using no activation function at all. Put differently, a bilinear layer replaces the activation function with a learnable matrix that operates in parallel, not sequence. 
One prominent question is: "If this is just doing linear operations, how can it be a good model component?".
While either side is linear, the whole is bilinear, which is non-linear to the input, which is all we need.

<div class="columns is-centered">
    <div class="column is-narrow">
        <figure>
        <img src="/research/bilinear/glu.svg" alt="GLU & Bilinear" />
        <figcaption> 
            An ordinary GLU (left) and a bilinear layer (right). An ordinary GLU has a gate part which "selects" <br> 
            which parts of the other side should be kept. In the case of a bilinear layer, this selection is instead continuous. 
        </figcaption>
        </figure>
    </div>
</div>

As foreshadowed, the linearity of each branch is helpful for interpretability. In this context, techniques from linear algebra, such as SVD, are now actually meaningful as they do not ignore the activation function. However, that's not all. Each output of a bilinear layer is equivalent to a sum of (weighted) pairwise input feature interactions. Note that this is impossible in a layer with a ReLU (or other activation functions); no simple formula describes how features interact without considering the inputs.

### Decomposing The Weights

Because of the bilinear layer's appealing properties, it becomes possible to trace from output to input, which was previously only possible using gradient-based methods. This allows us to decompose the model into the most important components for any desired output direction. While we won't go into details, these are the main ideas behind the decomposition.

1. The feature interactions of a bilinear layer output can be written as $\textbf{x}A\textbf{x}$ where we call $A$ the interaction matrix. Each entry in the matrix is the weight of how strongly the feature at the row and column should interact. Due to this structure, it is a symmetric matrix. While this is nice, this interaction matrix is complex to visualize for any non-trivial problem (especially in spatially dependent modalities, such as images).

2. We can leverage this symmetric property and perform an eigendecomposition on these matrices. This operation decomposes the matrix into $\sum_i \lambda_i v_i \otimes v_i$ (where $\otimes$ is an outer product) where values of $\lambda$ decrease. In other words, this finds which vectors best describe the interaction matrix. We can then easily visualize these vectors.

3. Since a sum of symmetric matrices is still symmetric, we can decompose any output direction, not just single outputs. For instance, we can take the difference between two features by subtracting the interaction matrices.

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

This shows that exploiting these properties of the bilinear layer can yield very interpretable structure from the weights alone. We use this towards multiple ends in the paper.

- Reverse-engineering an algorithmic task in an image classification model.
- Finding adversarial examples from the weights.
- Qualitatively studying the impact of regularization/augmentation.
- Post-hoc explanations of (mis)classification.

### Language Models

Each output of a bilinear layer is described by weighted pairwise interactions between their input
features. Beyond tracing between the actual output classes and the input, we can use this technique to trace between latent features in large models.
We use sparse dictionary learning, which can be seen as unsupervised probing, to create a semantically meaningful set of features (called a dictionary) around a hidden bilinear layer.
While such dictionary-based approaches have seen success in the past, they only indicate what features a model uses, not how they are used.
Using our method, we can understand the interactions between such dictionaries, based on the weights. This has several advantages.

- By finding the common structure between dictionary elements through eigenvectors, we partially resolve phenomena such as feature splitting.
- Understanding how a feature is formed gives more insight into what a feature actually does.

We apply this approach to a 6-layer language model, trained on children's stories.
These stories provide a more controlled environment compared to general datasets.

<div class="columns is-centered">
    <div class="column ">
        <img src="/research/bilinear/subspace.svg" alt="feature subspace" />
    </div>
</div>

We highlight one particular circuit that we extracted, which performs a form of sentiment negation.
This circuit performs an AND operation between sentiment features (firing broadly on positive and negative segments) and negation features (which fires on tokens which as not and isn't).
It then flips this sentiment in the output direction.

### Future Work

Due to the generality and novelty of this technique, we haven't yet explored many possibilities for interpretability. 
The current plan is to focus on continuing the work on language models and seeing which kinds of mechanisms we're able to detect.

<Cite text="@misc&#123;pearce2024bilinearmlpsenableweightbased,
      title=&#123;Bilinear MLPs enable weight-based mechanistic interpretability&#125;, 
      author=&#123;Michael T. Pearce and Thomas Dooms and Alice Rigg and Jose M. Oramas and Lee Sharkey&#125;,
      year=&#123;2024&#125;,
      eprint=&#123;2410.08417&#125;,
      archivePrefix=&#123;arXiv&#125;,
      primaryClass=&#123;cs.LG&#125;,
      url=&#123;https://arxiv.org/abs/2410.08417&#125;,
&#125;">
</Cite>
