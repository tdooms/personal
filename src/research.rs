use cobul::*;
use cobul::fa::Solid;
use yew::*;
use crate::image::Height;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ResearchData {
    name: &'static str,
    description: &'static str,
    start: &'static str,
    end: &'static str,
    image: &'static str,
    course: Option<&'static str>,
    source: Option<&'static str>,
    paper: Option<&'static str>,
    presentation: Option<&'static str>,
}

#[function_component(Research)]
pub fn research(props: &ResearchData) -> Html {
    let click = |url: Option<String>| {
        Callback::from(move |_| {
            if let Some(url) = url.clone() {
                web_sys::window().unwrap().open_with_url(&url).unwrap();
            }
        })
    };

    let paper_click = click(props.paper.map(ToString::to_string));
    let present_click = click(props.source.map(ToString::to_string));

    let d_paper = props.paper.is_none();
    let d_present = props.presentation.is_none();

    html! {
        <Columns>
        <Column size={ColumnSize::Is3}>
            <crate::image::DynImage src={props.image} height={Height::Vh(25)}/>
        </Column>
        <Column size={ColumnSize::Is7}>
        <Content>
            <Title size={HeaderSize::Is4}> {props.name} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start} {" - "} {props.end} </Subtitle>
            <span> {props.description} </span>
        </Content>
        </Column>
        <Column>
        <Buttons class="mt-6 mb-3 pt-5">
            <simple::Button icon={Solid::File} text="Paper" fullwidth=true outlined=true color={Color::Danger} disabled={d_paper} click={paper_click} />
            <simple::Button icon={Solid::PersonChalkboard} text="Presentation" fullwidth=true outlined=true color={Color::Danger} disabled={d_present} click={present_click} />
        </Buttons>
        </Column>
        </Columns>
    }
}