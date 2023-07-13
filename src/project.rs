use cobul::*;
use yew::*;
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
    let onclick = Callback::from(|_| {
        web_sys::window().unwrap().open_with_url(props.link).unwrap();
    });
    let icon = html! { <Icon icon={Brands::Github} size={Size::Large} />};

    html! {
        <div class="block" {onclick}>
        <Card class="my-project-item">
        <Content>
            <Media left={icon}>
            <Title size={HeaderSize::Is4}> {props.name} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.date} </Subtitle>
            </Media>
            <span>{props.description}</span>
        </Content>
        </Card>
        </div>
    }
}
