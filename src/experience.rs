use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ExperienceData {
    pub image: &'static str,

    pub start: &'static str,
    pub end: &'static str,

    pub company: &'static str,
    pub position: &'static str,

    pub comment: &'static str,
}

#[function_component(Experience)]
pub fn experience(props: &ExperienceData) -> Html {
    let image = html! {<Image size={ImageSize::Is48x48} src={props.image} class="m-0"/>};
    html! {
        <Block>
        <Card>
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.company} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start} {" - "} {props.end} </Subtitle>
            </Media>
            // <Title size={HeaderSize::Is5}> {props.position} </Title>
            <span>{props.comment}</span>
        </Content>
        </Card>
        </Block>
    }
}
