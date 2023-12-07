use cobul::*;
use yew::*;
use cobul::icons::Brands;
use implicit_clone::{ImplicitClone, unsync::IString};
use crate::util::redirect;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ProjectData {
    pub name: IString,
    pub link: IString,
    pub date: IString,
    pub description: IString,
}

impl ImplicitClone for ProjectData {}

#[function_component(Project)]
pub fn project(props: &ProjectData) -> Html {
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
