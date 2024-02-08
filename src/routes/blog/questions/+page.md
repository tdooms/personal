---
title: Beginner questions about deep learning
date: 11/01/2024
---

I've always been someone who has been interested in the why. I consider this the main reason for my fascination with science, it is easy to delve into a certain topic as far as possible, down to the axioms or fundamental laws governing the universe. When I started my university studies in computer science, this trend continued; I loved to learn about compiler internals, assembly, and even common hardware optimisations such as caching systems and branch prediction.

For instance, being a gamer since my childhood, I've always loved computer graphics. There is an irresistible appeal to the visual nature of this field. While it's bugs can be notoriously hard to track down, there is an inexplainable rush that comes from seeing all the maths and code come together into a beautiful virtual world. Furthermore, when studying this subject on my own in high school, I found tutorials really easy to follow, [especially this one on Vulcan](https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code). I contribute this to the fact that the field is relatively easy to structure into (problem -> algorithms). This results in, for example within Vulcan, that several algorithms or strategies are represented as an enum. Consequently, changing these values within the code is really trivial and its up- and downsides are usually quite clear. 

In contrast, the first time I looked at your typical neural networks introduction, I was very confused, almost all beginner tutorials were very high level and didn't explain any intricacies with the choices being made. However, at this stage, I thought to myself that I probably didn't have the correct resources or that I was missing some background, therefore I more or less ignored this until my first neural networks course at university. Here, the same high-level explanation was given, without significant attention to the why. Like many other initiates to the field, my biggest gripe where the seemingly arbitrary choices. These questions boiled down to: how the *bleep* did anyone come up with this? Or more importantly, why does any of this even work in the slightest? Even though all subsequent topics such as RNNs, YOLO, ResNets, and Transformers were easy to grasp. These fundamental questions continually gave me the idea that I didn't understand any of it.

Since then, I've gained a lot of insight within the field; I graduated my master's and successfully started my PhD within machine learning. Therefore, I would like to write to my past self (or others with the same predicament) and share small nuggets of knowledge that would've been extremely useful back then. However, even with a significant amount of expertise within the field, I've found that some of my questions were actually very justified as they're still not fully understood right now.

> **Disclaimer:** while most of these answers are generally correct, I have made some simplifications for ease of reading. Alternatively, I could simply be wrong, in which case, let me know.

That aside, let's start.

###### Why are neural networks structured in layers and not some other way?

This is one of the earliest questions that I figured out. The answer mostly boils down to three facts.
- Neural networks are biologically inspired, if we view the unstructured mess of neurons and synapses in the brain as a graph, this can be sorted into a layer-like structure. All neurons that receive some kind or sensory signal are the input layer, all neurons directly connected to this are the first layer and so on. This approach however, has some flaws, primarily the brain isn't a directed acyclic graph (DAG), which means there will be "back-edges" when performing this structuring. Within neural networks, these are simply removed because they are annoying to backpropagate through.
- Given this layerification of the brain, we can simply use an amount of layers equivalent to the diameter of the graph we wish to simulate (starting from the input). This guarantees that our network will be able to represent the full forward computation of the graph (e.g. the brain).
- The reason for this "overcomputation", is that its simply very efficient to implement these dense neural layers in hardware through matrix multiplication. Computers generally don't like sparse structures.

###### Why are biases used?

This has a very simple, satisfying explanation that I actually rarely see. Consider a black vs white image classification problem (very contrived I know, but bear with me). Within this classification problem, we have a fully black image (pixel values 0), as each layer is simply a matrix multiply, the whole subsequent network is fully 0. Which means the right prediction can never be made. That's why there is need for biases.

###### Why do we use activation functions? Why is ReLU better than sigmoid? 

Before that I want to quickly address a common misconception: activation functions are not used to somehow constrain the neuron. I saw this frequently as some kind motivation of sigmoid, clamping the value between -1 and 1. However, this explanation quickly breaks down when needing to explain why ReLU generally performs better than old-fashioned activation functions.

Instead, activation functions are necessary to prevent matrix collapse. Consider a network consisting of 2 layers without non-linearity, lets call the weights of these layers $A$ and $B$. Therefore, our final prediction is $\hat{y} = (x \cdot A) \cdot B$  , as matrices are associative, this can be written as $x \cdot (A \cdot B)$ . Now $A \cdot B$ could be replaced by some matrix $C$ that results in the same output. By using an activation function (or non-linearity as I prefer to call them) adding more layers actually makes sense. 

> Sometimes, we genuinely want to represent C as $A \cdot B$ (also known as a factorised matrix) to reduce the number of parameters. For instance if A has dimensions $100 \cdot 2$ and B has dimensions $2 \cdot 100$ this amounts to 400 parameters. In contrast, C would have 10 000. However, in neural networks, most matrices are square-ish, where there is no reason to decompose.

As for why ReLU is better, I haven't looked much into this but the main reason is that it doesn't suffer from vanishing gradients. Slightly related, it also allows the network to create sharper bounds by either scaling the activation quite high or near to zero.

###### How does one determine the architecture of a network, is there an arcane formula beyond  intuition or luck?

The short version: no, but it doesn't really matter. 

Longer version: Finding the smallest "circuit" (which is simply a term for a more general neural network) which for example guarantees >90% accuracy on some task is an NP complete problem and is absolutely untractable currently. However, in the spirit of pragmatism, we don't let this stop us and throw more compute at the problem to overcome this NP challenge. 

This can lead to two issues: underestimation and overestimation. Underestimating the complexity of a problem leads to a weak model that isn't able to "understand" the data, called underfitting. This can be explored on [this great website](https://playground.tensorflow.org).  On the other hand, when using too many layers, the phenomenon of overfitting (memorizing all data samples) becomes prevalent. 

When naively experimenting with neural networks, it seems difficult to avoid these regimes. I remember a time that I found it difficult to reach over 99.5% accuracy on MNIST, all my networks were peaking around the low 99% while you often hear that 99.8% is quite easy. A common response to this is that you simply need to pay close attention to the train/validation accuracies and try all kinds of setups empirically, which often seemed like a dumb response to me. How then did researchers come up with these exotic architectures such as ResNet, Inception, DenseNet and so forth, it must be more than sheer luck.

All of this seems like a big deal when first starting in the field, but it actually really isn't. Most times, it's actually very easy to find an architecture that performs very well through a rudimentary guesstimation. As for how these architectures are initially found, there is a funny (but deep down a little embarrassing) joke that architectures improvements are found using gradstudent descent. Meaning that simply lots of people are throwing ideas and designs to the walls and seeing what sticks best.

You may be thinking: "I thought this was a post about more fundamentally understanding neural networks and the author is simply brushing away the question with dumb jokes" and you'd be very correct. However, actually fully understanding the complexity of the situation requires quite a bit of effort and doesn't lead to a satisfying response.

For example, you may know dropout; this deactivates neurons randomly with the intention of forcing the network to be more robust to perturbances and therefore rely less on random patterns. There is also weight decay, which forces weights to generally be smaller and therefore avoid over-reliance on specific features which may not generally hold. These techniques under the umbrella of regularizers. However,  [an underappreciated paper](https://arxiv.org/abs/1611.03530) disproves this. They shuffle the labels of the whole training set, meaning all image-label pairs are deterministic but simply form a random pattern. Therefore, to achieve high accuracy, the network has no other choice than to overfit. However, the authors observe that most models can achieve near-perfect accuracy on training but even worse, that strongly regularized models can do so too. Now, the astute reader may make the observation that, even though there is some regularization, the model has no other choice but that within a normal regime the network will simply work better. However, this claim has also been disputed by a wide range of papers.

However, while there is a lot of discussion about what neural networks are learning and how to change this. There are some undisputed insights and rules of thumb about the expressivity (the theoretical things they can learn and represent) of models.

- The most straightforward is the universal approximation theorem, this states than an infinitely wide 1-layer network (with non-linearity) can represent any possible function. I like to think about this as constructing the whole function in a piecewise manner (maybe another reason why ReLU > sigmoid). 
- In the other extreme case, it's not difficult to realise that a very deep 1-neuron wide network is quite worthless.
- However, mathematically, scaling by depth is preferred as they exponentially improve the expressivity, while scaling width is only quadratic. There are some mathematical results in this area that determine optimal ratios, however, I haven't looked into them a bunch because they don't really reveal anything interesting.

Anyway, long story short, even highly regarded academics don't know the answer to this question, barring trivially simple, synthetic problems.

###### Why does the network seem to learn structures that make sense?

The first time I saw a result of this kind was in a lecture that showed a t-SNE plot of features throughout layers with the samples coloured by the ground truth class. In essence, the network separates the features of samples a bit further at each stage of the model. At the time, this was completely counter-intuitive to me, I expected complete chaos up until the last layer (a regime more akin to counting sort). Now, I realise that this behaviour is quite logical. Concretely, working backwards, the last hidden layer should be linearly separable for good classification. Then, inductively, the more separable the previous layer is, the more separable the next one is. In this framework, we can mostly ignore the ReLU as it is only affects half the features. 

> There are some fun videos visualising sorting algorithms. In most cases, you can slowly see the data being organised but in counting sort or radix sort, the shuffle seems nonsensical until it is magically fully sorted.

But beyond this separability, networks tend to (mostly) produce features that are understandable by humans or adhere to easy-to-follow intuitions. It's been shown that larger models, just like the human brain have a "Trump" neuron. In multi-modal settings, this fires when seeing an image of him or reading the name or even something strongly related to it. As far as I know, no concrete arguments exist as to why this happens, similar to humans, encoding these concepts seems to be an optimal way of doing stuff.