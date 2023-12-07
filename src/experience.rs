use cobul::*;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ExperienceData {
    pub image: IString,

    pub start: IString,
    pub end: IString,

    pub company: IString,
    pub position: IString,

    pub comment: IString,
}

impl ImplicitClone for ExperienceData {}

#[function_component(Experience)]
pub fn experience(props: &ExperienceData) -> Html {
    let image = html! {<Image size={ImageSize::Is48x48} src={props.image.clone()} class="m-0"/>};

    html! {
        <Block>
        <Card>
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.company.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start.clone()} {" - "} {props.end.clone()} </Subtitle>
            </Media>
            // <Title size={HeaderSize::Is5}> {props.position} </Title>
            <span> {props.comment.clone()} </span>
        </Content>
        </Card>
        </Block>
    }
}
