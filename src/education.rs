use cobul::*;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct EducationData {
    pub image: IString,

    pub start: IString,
    pub end: IString,

    pub institution: IString,
    pub subject: IString,
    pub grade: Option<IString>,
    pub topic: Option<IString>,
    pub advisor: Option<IString>,
}

impl ImplicitClone for EducationData {}

#[function_component(Education)]
pub fn education(props: &EducationData) -> Html {
    let image = html! {<Image size={ImageSize::Is48x48} src={props.image.clone()} class="m-0"/>};

    let grade = match props.grade.clone() {
        Some(grade) => html! {<p class="mb-1"><b> {"Grade: "}</b> {grade}</p>},
        None => html! {},
    };

    let topic = match props.topic.clone() {
        Some(topic) => html! {<p class="mb-1"><b> {"Topic: "}</b> {topic}</p>},
        None => html! {},
    };

    let advisor = match props.advisor.clone() {
        Some(advisor) => html! {<p class="mb-1"><b> {"Advisor: "}</b> {advisor}</p>},
        None => html! {},
    };

    html! {
        <Block>
        <Card>
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.institution.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start.clone()} {" - "} {props.end.clone()} </Subtitle>
            </Media>
            <Title size={HeaderSize::Is5} class="mb-3"> {props.subject.clone()} </Title>
            {topic}
            {advisor}
            {grade}
        </Content>
        </Card>
        </Block>
    }
}
