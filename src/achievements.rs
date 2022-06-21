use cobul::props::{Color, ColumnSize, Size};
use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct AchievementData {
    pub text: &'static str,
}

#[function_component(Talent)]
pub fn achievements(props: &AchievementData) -> Html {
    html! {
        <IconText>
        <Icon icon="fas fa-trophy"/>
        <span> {props.text} </span>
        </IconText>
    }
}
