use cobul::props::{Color, Size};
use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct TalentData {
    pub name: &'static str,
}

#[function_component(Talent)]
pub fn talent(props: &TalentData) -> Html {
    html! {
        <Tag size={Size::Large} color={Color::Light}> {props.name} </Tag>
    }
}
