use cobul::*;
use yew::*;
use yew_router::prelude::*;
use cobul::fa::Brands;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ProjectData {
    pub name: &'static str,
    pub link: &'static str,
    pub date: &'static str,
    pub description: &'static str,
}

#[function_component(Project)]
pub fn project(props: &ProjectData) -> Html {
    let navigator = use_navigator().unwrap();

    let click = Callback::from(|_| {
        // let _ = web_sys::window().unwrap().open_with_url(props.link);
    });
    let icon = html! { <Icon icon={Brands::Github} {click} size={Size::Large} style="cursor:pointer" />};

    html! {
        <Block>
        <Card>
        <Content>
            <Media left={icon}>
            <Title size={HeaderSize::Is4}> {props.name} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.date} </Subtitle>
            </Media>
            <span>{props.description}</span>
        </Content>
        </Card>
        </Block>
    }
}
