use gloo::timers::callback::Timeout;
use yew::*;
use cobul::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub done: Callback<()>,
}

#[function_component(Start)]
pub fn start(props: &Props) -> Html {
    let done = props.done.clone();

    let countdown = use_state(|| 3);
    let timer = use_state(|| Timeout::new(0, || ()));

    use_effect_with(countdown.clone(), move |countdown| match **countdown{
        0 => done.emit(()),
        value => {
            let countdown = countdown.clone();
            timer.set(Timeout::new(1_000, move || countdown.set(value - 1)))
        },
    });

    html! {
        <>
        <Hero size={HeroSize::Medium} class="has-text-centered">
            <Title style="font-size:60px" size={HeaderSize::Is1}> {countdown.to_string()} </Title>
        </Hero>
        </>
    }
}