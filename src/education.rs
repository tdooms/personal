use cobul::props::{Color, HeaderSize, ImageSize};
use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct EducationData {
    pub image: &'static str,

    pub start: &'static str,
    pub end: &'static str,

    pub institution: &'static str,
    pub subject: &'static str,
    pub grade: &'static str,
}

#[function_component(Education)]
pub fn education(props: &EducationData) -> Html {
    let image = html! {<Image size={ImageSize::Is48x48} src={props.image} class="m-0"/>};

    html! {
        <Block>
        <Card>
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.institution} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start} {" - "} {props.end} </Subtitle>
            </Media>
            <Title size={HeaderSize::Is5}> {props.subject} </Title>
            <span><b> {"Grade: "}</b> {props.grade}</span>
        </Content>
        </Card>
        </Block>
    }
}
