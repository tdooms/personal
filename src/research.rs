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
    source: &'static str,
    image: &'static str,
    paper: Option<&'static str>,
    video: Option<&'static str>,
}

#[function_component(Research)]
pub fn research(props: &ResearchData) -> Html {
    let footer = html! {
        <>
        <a href={props.paper}> {"Paper"} </a>
        <a href={props.video}> {"Video"} </a>
        </>
    };

    html! {
        <Columns>
        <Column size={ColumnSize::Is3}>
            <crate::image::DynImage src={props.image} height={Height::Vh(25)}/>
        </Column>
        <Column size={ColumnSize::Is7}>
        <Content>
            <Title size={HeaderSize::Is4}> {props.name} </Title>
            <Subtitle size={HeaderSize::Is6}> {props.start} {" - "} {props.end} </Subtitle>
        <span>{props.description}</span>
        </Content>
        </Column>
        <Column>
        <Buttons class="mt-6 mb-3">
        <simple::Button icon={Solid::File} text="Paper" fullwidth=true outlined=true color={Color::Danger}/>
        <simple::Button icon={Solid::Video} text="Video" fullwidth=true outlined=true color={Color::Danger} />
        <simple::Button icon={Solid::Link} text="Source" fullwidth=true outlined=true color={Color::Danger} />
        </Buttons>
        </Column>
        </Columns>
    }
}