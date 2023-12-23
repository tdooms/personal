use cobul::*;
use cobul::icons::Solid;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};
use crate::image::Height;
use crate::util::redirect;

#[derive(Properties, PartialEq, Clone, Debug)]
struct ReferenceProps {
    url: Option<IString>,
    text: IString,
    icon: Option<IString>,
}

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ResearchData {
    pub name: IString,
    pub description: IString,
    pub kind: IString,
    pub start: IString,
    pub end: IString,
    pub image: IString,
    pub course: Option<IString>,
    pub source: Option<IString>,
    pub paper: Option<IString>,
    pub presentation: Option<IString>,
}

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub research: IArray<ResearchData>,
}

impl ImplicitClone for ResearchData {}

#[function_component(Reference)]
fn reference(props: &ReferenceProps) -> Html {
    let ReferenceProps { url, text, icon } = props.clone();

    let disabled = url.is_none();
    let click = url.map(redirect).unwrap_or_default();

    html! {
        <Button {icon} {text} fullwidth=true outlined=true color={Color::Danger} {disabled} {click} />
    }
}

#[function_component(Topic)]
fn topic(props: &ResearchData) -> Html {
    html! {
        <Columns>
        <Column size={ColumnSize::Is3}>
            <crate::image::DynImage src={props.image.clone()} height={Height::Vh(25)}/>
        </Column>
        <Column size={ColumnSize::Is7}>
        <Content>
            <Title size={HeaderSize::Is4}> {props.name.clone()} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start.clone()} {" - "} {props.end.clone()} {" ("}<i>{props.kind.clone()}</i> {")"} </Subtitle>
            <span> {props.description.clone()} </span>
        </Content>
        </Column>
        <Column>
        <Buttons class="mt-6 mb-3 pt-5">
            <Reference icon={Solid::File} text="Paper" url={props.paper.clone()} />
            <Reference icon={Solid::PersonChalkboard} text="Presentation" url={props.presentation.clone()} />
        </Buttons>
        </Column>
        </Columns>
    }
}

#[function_component(Research)]
pub fn research(props: &Props) -> Html {
    html! { for props.iter().map(|data| html! {<Topic ..data />}) }
}