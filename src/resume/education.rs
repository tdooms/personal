use cobul::*;
use yew::*;
use implicit_clone::ImplicitClone;


use super::ResumeData;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct EducationData {
    pub image: AttrValue,

    pub start: AttrValue,
    pub end: AttrValue,

    pub institution: AttrValue,
    pub title: AttrValue,

    pub grade: Option<AttrValue>,
    pub advisor: Option<AttrValue>,
}

// #[derive(Properties, PartialEq, Clone, Debug)]
// pub struct Props {
//     pub education: IArray<EducationData>,
// }

impl ImplicitClone for EducationData {}

#[function_component(Item)]
fn item(props: &EducationData) -> Html {
    let image = html! { <Image size={ImageSize::Is48x48} src={props.image.clone()} class="m-0"/> };

    let kv = |name: &str, value: Option<AttrValue>| match value {
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
pub fn education(resume: &ResumeData) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Education"} </Title>
        { for resume.education.iter().map(|data| html! {<Item ..data />}) }
        </>
    }
}

