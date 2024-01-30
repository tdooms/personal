use cobul::*;
use yew::*;
use implicit_clone::{ImplicitClone};

use super::ResumeData;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ExperienceData {
    pub image: AttrValue,

    pub start: AttrValue,
    pub end: AttrValue,

    pub company: AttrValue,
    pub position: AttrValue,

    pub comment: AttrValue,
}

impl ImplicitClone for ExperienceData {}

#[function_component(Item)]
fn item(props: &ExperienceData) -> Html {
    let image = html! {<Image size={ImageSize::Is48x48} src={props.image.clone()} class="m-0"/>};

    html! {
        <Block>
        <Card style="height: 200px">
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.company.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start.clone()} {" - "} {props.end.clone()} </Subtitle>
            </Media>
            <span> {props.comment.clone()} </span>
        </Content>
        </Card>
        </Block>
    }
}

#[function_component(Experience)]
pub fn experience(resume: &ResumeData) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Experience"} </Title>
        { for resume.experience.iter().map(|data| html! {<Item ..data />}) }
        </>
    }
}
