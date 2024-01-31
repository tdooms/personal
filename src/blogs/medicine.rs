use cobul::*;
use yew::*;

use super::footnote::{use_footnote_context, Footnote};


#[function_component(Medicine)]
pub fn medicine() -> Html {
    let ctx = use_footnote_context(3);

    let title = "An analogy of neural networks and the human body";

    let p1 = "The reason for this post is to provide an intuitive explanation of mechanistic interpretability 
    through an analogy to medicine. It is (hopefully) easy to follow without too much jargon. This way, if people 
    ask what I am researching, I can refer them to this post. If this is you, hi again!";

    let p2 = "This post compares the evolution of the field of medicine to mechanistic interpretability. 
    While thinking about this analogy, I found that the historic endeavours to understand the human body bear a 
    surprising resemblance to our current quest of understanding neural networks. Therefore, I wanted to write 
    up the similarities and attempt to learn some lessons from its process. Sadly, I know nothing about medicine 
    so do not take any related claims too seriously.";

    let p3 = "The quest to understand the body mostly originated from the need to heal people. However, 
    in ancient times, this practice was riddled with misconceptions due to a lack of knowledge. People were 
    mostly led by intuition, spirituality, and guesswork. One interesting hypothesis is humourism. Mostly 
    believed by the ancient Greeks, this idea posits that the body consists of 4 basic fluids (blood, phlegm, 
    yellow- and black bile) that need to be kept in balance for good health. A lack or excess of any of 
    these fluids was often given as a reason for mood swings or illness. Gruesomely (easy to say in 
    hindsight of course), this theory was put to practice with techniques like bleeding which often 
    did more harm than good. Around the 17th century, these theories began falling out of favour. 
    The rise of the scientific method and proper instrumentation led to new discoveries such as germ 
    theory and modern physiology. Since then, our understanding of the human body and advanced medicine 
    has only improved and has augmented our quality of life significantly.";

    let p4 = "While our predecessors can be excused for their misbelief, I strongly feel that similar
    mistakes are being made in the quest to understand deep neural networks (aka interpretability). Let us zoom 
    in on some (subjective) mistakes made throughout history in medicine and propose analogues within interpretability. 
    Afterwards, an outline of a promising solution is given.";

    let st1 = "A strong focus on the surface, disregarding the internals.";
    
    let fn1 = "The study of adversarial examples has been immensely useful in protecting models from malicious input in practice. 
    However, one must admit our lack of understanding towards their origin. Famously, adversarial defence mechanisms can be easily 
    obviated by using slightly different techniques.";

    let p5 = html! {<p>{"The internals of the human body are confusing, without our current knowledge of organs and their 
    function, it is easy to understand how the insides of beings were often disregarded in favour of more intuitive 
    theories. The same holds for neural networks, it seems we do not understand how its internals work, except for 
    the input and output layers. Consequently, a considerable amount of research focuses on this surface, such as 
    adversarial examples which confuse current models by perturbing the input with (humanly) imperceptible noise. 
    However, similar to our previous story, any experiments in this domain remain quite high-level and do not 
    really provide any insight"} <Footnote ctx={ctx.clone()} text={fn1}/> {". Drawing conclusions 
    from the surface of such a complex (and sometimes illogical) system will inevitably lead to false conclusions."}</p>};

    let st2 = "Inadequate rigour and validation";

    let p6 = "When reading further about humourism, I was astounded as to why no one actually properly tested this hypothesis. 
    While there is probably lots more to this that I don't know about, the simple fact that \"it seems reasonable\" and provided 
    a believable explanation was enough to justify its prevalence. In fact, the same often happens in modern academia, where results 
    are often not verified, even when reproduction is not necessarily difficult (such as the superconductivity of LK-99). 
    This is even the case within machine learning, where exact code can be shared and executed to validate claims. Yet, 
    I've found myself using the \"yeap, that looks about right\" metric quite a lot when looking at most graphs and numbers in papers.";

    let st3 = "Absence of quantitative and causal experimentation";

    let p7 = "Beyond the justification for negligence of rigour within science (ancient and modern), the root cause of 
    false theories is probably a lack of tooling. The ancient Greeks didn't have a microscope so how could they ever predict 
    that microbes were the primary cause of illness? Furthermore, the scientific virtues, to guide their experiments, were not 
    as developed back then. Within machine learning, we have full control and inspection capabilities of the full digital environment. 
    There are no limitations other than our intelligence. Therefore, even though the field of machine learning is young, there is no 
    excuse for sloppy science.";
    
    let fn2 = "The motivations of most papers in this domain refer to the faulty horse detector, that heatmaps helped detect. 
    However, this was also apparent from simpler factors such as the models increased confidence in detecting horses. Let me provide 
    context; there was a model trained on a dataset in which almost all images of horses, exclusively, had a date on the bottom 
    (an artefact from the camera presumably). By looking at the heatmaps, researchers found out the model was simply looking at
    the existence of the date to classify something as a horse.";

    let fn3 = "Heatmaps are not the only offender in this area, many highly acclaimed papers (which I won't explicitly name) 
    have the same issue. While their academic significance is unquestionable, they should only be used for intuitive purposes rather 
    than actual interpretability.";
    
    let p8 = html!{<p>{"Still, a large majority of the field of interpretability is qualitative. A prominent example is heatmaps, which 
    aim to depict which area of an image a model looks at most. The idea is that we, as humans, can then evaluate if the model 
    is 'doing what we want'"} 
    <Footnote ctx={ctx.clone()} text={fn2} /> 
    {". My main objections to this approach are: \"What would this heatmap even look like\" and more 
    importantly: \"What knowledge would we gain given a perfect heatmap\". Importantly, a heatmap is simply a proxy for 
    interpretability. The statement \"if the model only looks at the cat, it must understand that it is a cat and therefore 
    predict that\" is a logical fallacy. The model may well fully highlight the cat, but if the fur of the cat is a bit too 
    monkey-like, it may predict wrong, and we have no way of noticing this"}
    <Footnote ctx={ctx.clone()} text={fn3} />
    {"."} </p>}; 

    let st4 = "The struggle to find the right scale for observation";

    let p9 = "Given a complex system like a human, how does one even start to analyse this? Should we look at the composition 
    of limbs, cells or even atoms to understand its function? Medicine has the advantage that most biological systems are 
    surprisingly modular. Organs seem to be neatly separated by membranes and even within organs, there is lots of structure. 
    For instance, lungs are conceptually just a tree structure that guides air towards small alveoli. This naturally defines a 
    conceptual space for understanding the function of items.";

    let p10 = "Within machine learning, it seems more difficult to find such a granularity. Conceptually, we design neural 
    networks layer by layer but these layers do not seem very interpretable. Some high-level approaches attempt to project 
    the neuron activations (the data of each layer) onto a 2D plane, this shows that the network slowly learns to separate 
    the data into distinct groups given its class (the answer). Comparably to previous approaches, this doesn't reveal the 
    exact working of the networks.";
    
    let p11 = "Another approach is to dive deeper: onto the neuron level. The large body of 
    research in this domain tells a semi-optimistic story. Some neurons are highly interpretable, activating on concrete 
    concepts such as cars or dogs. However, beyond these cherry-picked few, there are many \"poly-semantic\" neurons which 
    activate on completely unrelated stuff such as leaves and dice. Further, there are also some noise neurons which do not 
    follow any coherent structure. In a sense this is logical, neurons need not represent human concepts. However, this renders 
    the interpretability quite difficult.";

    let st5 = "Solution";

    let p12 = "Even though this explanation portrays the field of interpretability in a bad light, there is reason 
    for hope. Given the current flaws, there is a strong trend towards a new form of interpretability: mechanistic interpretability. 
    This field aims to \"reverse engineer\" a network, decomposing it into parts. This decomposition may be unfathomably complex, but 
    it will provide a way to causally reason about models. It is achieved through a collection of low-level methods that are able to 
    comprehensively describe certain parts of the model (which I plan to write another blog about later).";

    let p13 = "Causally reasoning about the internals of any system is an exceedingly useful tool. For example, in biology, 
    we can reason that removing an appendix has almost no causal effect on the proper functioning of the body (apparently this 
    is not fully true, but whatever). These organs, which lost their function throughout evolution are called vestigial organs. 
    It is not unexpected that neural networks may also have these vestigial subsections or neurons that once served a purpose 
    during training but have been mostly replaced. If this proportion is significant, they could be removed to speed up the model. 
    Beyond this, if we can causally reason about knowledge within a model, we could perform a slight update to update the model's 
    beliefs. Instead of attempting to rid training data of the model of racism or another unwanted human by-products, we can 
    identify these sections and lobotomise (remove) them. Further, through causal reasoning, we can learn from models through 
    the extraction of useful information in a chess AI for example. Lastly, we can use causal reasoning to identify failure 
    cases within networks, which is immensely useful in highly sensitive industries for example.";

    let p14 = "To reach real interpretability, we require more than simply the toolbox of mechanistic interpretability. 
    I argue that there are two additional vital components in this endeavour: interactive dissemination and standardised 
    databases. The first aims to increase the standard to which interpretability techniques are held. Interactive 
    visualisations, for example, should allow users to browse through the presented claims on a specific model. Specifically, 
    this avoids the shoddy scientific practice of cherry-picking. The second aims to propose a certain platform for common 
    knowledge much like was done to uncover the human genome. If done right, this will leave room for the creative, scientific 
    process but provide a sort of atlas of interpretability across several models.";

    let p15 = "In conclusion, there is a lot to learn from our journey through the understanding of the human body. 
    We should exploit the scientific method that has been established throughout multiple fields in our quest to understand 
    neural networks. Finally, we should share our acquired insights through interactive visualisations and shared knowledge atlasses.";

    html! {
        <Content style="width:700px">
            <Title size={HeaderSize::Is4}> {title} </Title>
            <p> {p1} </p>
            <p> {p2} </p>
            <p> {p3} </p>
            <p> {p4} </p>
            <b> {st1} </b>
            <p> {p5} </p>
            <b> {st2} </b>
            <p> {p6} </p>
            <b> {st3} </b>
            <p> {p7} </p>
            <p> {p8} </p>
            <b> {st4} </b>
            <p> {p9} </p>
            <p> {p10} </p>
            <p> {p11} </p>
            <b> {st5} </b>
            <p> {p12} </p>
            <p> {p13} </p>
            <p> {p14} </p>
            <p> {p15} </p>
        </Content>
    }
}