use yew::*;
use cobul::*;
use serde::Deserialize;
use implicit_clone::{unsync::IString};

#[derive(Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ProfileData {
    name: IString,
    location: IString,
    profession: IString,
    picture: IString,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileData) -> Html{
    let ProfileData {name, location, profession, picture} = props.clone();

    html! {
        <>
        <div class="has-text-centered">
        <Image size={ImageSize::Is128x128} rounded=true src={picture} class="is-inline-block"/>
        </div>

        <Subtitle size={HeaderSize::Is4} class="has-text-centered mb-3"> {name} </Subtitle>
        <Subtitle size={HeaderSize::Is6} class="has-text-centered has-text-grey"> {location} </Subtitle>

        <Block>
        <Title size={HeaderSize::Is5} class="has-text-centered"> {profession} </Title>
        </Block>

        <hr class="has-background-black"/>
        </>
    }
}