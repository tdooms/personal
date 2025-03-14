---
title: Introducing SimpleStories
date: 12 Mar 2024
kind: research
---

<script>
  import Resources from "$lib/research/resources.svelte";
  import Cite from "$lib/research/cite.svelte"
</script>

<p> Lennart Finke, Juan Diego Rodriguez, <b>Thomas Dooms</b>, Mat Allen, Thomas Marshall, Noa Nabeshima, Dan Braun  </p>

<div class="mt-6"> </div>

<Resources paper="https://openreview.net/attachment?id=JO8CtTXOsH&name=pdf"/>

### Motivation

To interpret deep neural networks, one needs to answer two questions simultaneously: "what to look for and how?".
It is both unknown what deep model's internal mechanisms are and how they are represented.
This stems for a lack of structure in most datasets, containing random scraps of data from the internet.
Hence, there is increasing need for structured datasets that can help guide interpretability research toward concrete goals.

One milestone toward this is [TinyStories](https://arxiv.org/abs/2305.07759), a dataset containing millions of children's stories.
Tiny language models (~10 million parameter) trained on this dataset can generate coherent and creative stories.
The combination of tiny models and constrained problem space forms a perfect testbed for understanding these models.

Unfortunately, this dataset suffers from a few issues:

- Stories are generated using old models, leading to formulaic and/or incoherent plots.
- Some stories are 'corrupted', containing large amounts of nonsense characters.
- The vocabulary isn't constrained enough, leading to complex or even misspelled words.

Furthermore, there are ample opportunities for improvement:

- Fine-grained labels about aspects of the story (tone, moral, topic) that can serve for supervised probing/finetuning.
- Strong focus on simplicity, while retaining diversity.

Keeping this in mind, we created SimpleStories.

<Cite text="@inproceedings&#123;
    finke2025tiny,
    title=&#123;[Tiny] Parameterized Synthetic Text Generation with SimpleStories&#125;,
    author=&#123;Lennart Finke and Juan Diego Rodriguez and Thomas Dooms and Mat Allen and Thomas Marshall and Noa Nabeshima and Dan Braun&#125;,
    booktitle=&#123;Will Synthetic Data Finally Solve the Data Access Problem?&#125;,
    year=&#123;2025&#125;,
    url=&#123;https://openreview.net/forum?id=JO8CtTXOsH&#125;
&#125;" />