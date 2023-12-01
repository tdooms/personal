use cobul::*;
use implicit_clone::unsync::IString;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct TraitData {
    pub name: IString,
}

#[function_component(Trait)]
pub fn trait0(props: &TraitData) -> Html {
    html! {
        <Tag size={Size::Large} color={Color::Light}> {props.name.clone()} </Tag>
    }
}
