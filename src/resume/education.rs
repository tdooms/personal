use cobul::*;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};
use implicit_clone::unsync::IArray;
use crate::resume::CvProps;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct EducationData {
    pub image: IString,

    pub start: IString,
    pub end: IString,

    pub institution: IString,
    pub title: IString,

    pub grade: Option<IString>,
    pub advisor: Option<IString>,
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub education: IArray<EducationData>,
}

impl ImplicitClone for EducationData {}

#[function_component(Item)]
fn item(props: &EducationData) -> Html {
    let image = html! { <Image size={ImageSize::Is48x48} src={props.image.clone()} class="m-0"/> };

    let kv = |name: &str, value: Option<IString>| match value {
        Some(value) => html! {<p class="mb-1"><b> {name} {": "}</b> {value}</p>},
        None => html! {},
    };

    html! {
        <Block>
        <Card style="height: 200px">
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.institution.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start.clone()} {" - "} {props.end.clone()} </Subtitle>
            </Media>
            <Title size={HeaderSize::Is5} class="mb-3"> {props.title.clone()} </Title>
            {kv("Advisor", props.advisor.clone())}
            {kv("Grade", props.grade.clone())}
        </Content>
        </Card>
        </Block>
    }
}

#[function_component(Education)]
pub fn education(CvProps{cv}: &CvProps) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Education"} </Title>
        { for cv.education.iter().map(|data| html! {<Item ..data />}) }
        </>
    }
}

