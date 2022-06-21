use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct AchievementData {
    pub text: &'static str,
}

#[function_component(Achievement)]
pub fn achievement(props: &AchievementData) -> Html {
    html! {
        <p>
        <IconText>
        <Icon icon="fas fa-trophy"/>
        <span> {props.text} </span>
        </IconText>
        </p>
    }
}
