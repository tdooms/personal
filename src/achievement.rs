use cobul::*;
use implicit_clone::unsync::IString;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct AchievementData {
    pub text: IString,
}

#[function_component(Achievement)]
pub fn achievement(props: &AchievementData) -> Html {
    html! {
        <p>
        <IconText>
        <Icon icon="fas fa-trophy"/>
        <span> {props.text.clone()} </span>
        </IconText>
        </p>
    }
}
