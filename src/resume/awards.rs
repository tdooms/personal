use cobul::*;
use implicit_clone::unsync::IString;
use yew::*;
use crate::resume::CvProps;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
struct AwardData {
    pub text: IString,
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
pub fn awards(CvProps{cv}: &CvProps) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Honours and Awards"} </Title>
        { for cv.awards.iter().map(|text| html! {<Item {text} />}) }
        </>
    }
}
