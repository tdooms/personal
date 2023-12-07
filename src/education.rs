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


#[derive(Properties, PartialEq, Clone, Debug)]
struct ItemProps {
    pub name: IString,
    pub value: Option<IString>
}

#[function_component(Item)]
fn item(props: &ItemProps) -> Html {
    match props.value.clone() {
        Some(value) => html! {<p class="mb-1"><b> {props.name.clone()} {": "}</b> {value}</p>},
        None => html! {},
    }
}

#[function_component(Education)]
pub fn education(props: &EducationData) -> Html {
    let image = html! { <Image size={ImageSize::Is48x48} src={props.image.clone()} class="m-0"/> };

    html! {
        <Block>
        <Card>
        <Content>
            <Media left={image}>
            <Title size={HeaderSize::Is4}> {props.institution.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start.clone()} {" - "} {props.end.clone()} </Subtitle>
            </Media>
            <Title size={HeaderSize::Is5} class="mb-3"> {props.subject.clone()} </Title>
            <Item name="Topic" value={props.topic.clone()} />
            <Item name="Advisor" value={props.advisor.clone()} />
            <Item name="Grade" value={props.grade.clone()} />
        </Content>
        </Card>
        </Block>
    }
}
