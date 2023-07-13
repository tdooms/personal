use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct EducationData {
    pub image: &'static str,

    pub start: &'static str,
    pub end: &'static str,

    pub institution: &'static str,
    pub subject: &'static str,
    pub grade: Option<&'static str>,
    pub topic: Option<&'static str>,
}

#[function_component(Education)]
pub fn education(props: &EducationData) -> Html {
    let image = html! {<Image size={ImageSize::Is48x48} src={props.image} class="m-0"/>};

    let grade = match props.grade {
        Some(grade) => html! {<span><b> {"Grade: "}</b> {grade}</span>},
        None => html! {},
    };

    let topic = match props.topic {
        Some(topic) => html! {<span><b> {"Topic: "}</b> {topic}</span>},
        None => html! {},
    };

    html! {
        <Block>
        <Card>
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.institution} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start} {" - "} {props.end} </Subtitle>
            </Media>
            <Title size={HeaderSize::Is5} class="mb-3"> {props.subject} </Title>
            {topic}
            {grade}
        </Content>
        </Card>
        </Block>
    }
}
