use cobul::*;
use yew::*;
use cobul::icons::Brands;
use implicit_clone::{ImplicitClone, unsync::IString, unsync::IArray};
use crate::resume::CvProps;
use crate::util::redirect;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ProjectData {
    pub name: IString,
    pub link: IString,
    pub date: IString,
    pub description: IString,
}

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub title: IString,
    pub projects: IArray<ProjectData>,
}

impl ImplicitClone for ProjectData {}

#[function_component(Item)]
pub fn item(props: &ProjectData) -> Html {
    let onclick = redirect(props.link.clone()).reform(|_| ());
    let icon = html! { <Icon icon={Brands::Github} size={Size::Large} />};

    html! {
        <div class="block" {onclick}>
        <Card class="my-project-item">
        <Content>
            <Media left={icon}>
            <Title size={HeaderSize::Is4}> {props.name.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.date.clone()} </Subtitle>
            </Media>
            <span>{props.description.clone()}</span>
        </Content>
        </Card>
        </div>
    }
}

#[function_component(Personal)]
pub fn personal(CvProps{cv}: &CvProps) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Personal Projects"} </Title>
        { for cv.personal.iter().map(|data| html! {<Item ..data />}) }
        </>
    }
}

#[function_component(Academic)]
pub fn academic(CvProps{cv}: &CvProps) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Academic Projects"} </Title>
        { for cv.academic.iter().map(|data| html! {<Item ..data />}) }
        </>
    }
}
