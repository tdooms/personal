use cobul::props::{Color, Size};
use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ProjectData {
    pub text: &'static str,
}

#[function_component(Project)]
pub fn project(props: &ProjectData) -> Html {
    html! {
        <Notification> {props.text} </Notification>
    }
}
