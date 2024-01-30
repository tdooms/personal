use cobul::*;
use yew::*;

use super::ResumeData;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
struct AwardData {
    pub text: AttrValue,
}

#[function_component(Item)]
fn item(props: &AwardData) -> Html {
    html! {
        <p>
        <IconText>
        <Icon icon="fas fa-trophy"/>
        <span> {props.text.clone()} </span>
        </IconText>
        </p>
    }
}


#[function_component(Awards)]
pub fn awards(resume: &ResumeData) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Honours and Awards"} </Title>
        { for resume.awards.iter().map(|text| html! {<Item {text} />}) }
        </>
    }
}
