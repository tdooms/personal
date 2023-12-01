use cobul::*;
use yew::*;
use cobul::icons::Brands;
use implicit_clone::{ImplicitClone, unsync::IString};

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
    let link = props.link.clone();

    let onclick = Callback::from(move |_| {
        web_sys::window().unwrap().open_with_url(&link).unwrap();
    });

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
