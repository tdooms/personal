use cobul::*;
use cobul::icons::Solid;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};
use crate::image::Height;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ResearchData {
    pub name: IString,
    pub description: IString,
    kind: IString,
    pub start: IString,
    pub end: IString,
    pub image: IString,
    pub course: Option<IString>,
    pub source: Option<IString>,
    pub paper: Option<IString>,
    pub presentation: Option<IString>,
}

impl ImplicitClone for ResearchData {}

#[function_component(Research)]
pub fn research(props: &ResearchData) -> Html {
    let click = |url: Option<AttrValue>| {
        Callback::from(move |_| {
            if let Some(url) = url.clone() {
                web_sys::window().unwrap().open_with_url(&url).unwrap();
            }
        })
    };

    let paper_click = click(props.paper.clone());
    let present_click = click(props.presentation.clone());

    let d_paper = props.paper.is_none();
    let d_present = props.presentation.is_none();

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
            <Button icon={Solid::File} text="Paper" fullwidth=true outlined=true color={Color::Danger} disabled={d_paper} click={paper_click} />
            <Button icon={Solid::PersonChalkboard} text="Presentation" fullwidth=true outlined=true color={Color::Danger} disabled={d_present} click={present_click} />
        </Buttons>
        </Column>
        </Columns>
    }
}