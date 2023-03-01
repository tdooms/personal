use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ResearchData {
    name: &'static str,
    description: &'static str,
    span: &'static str,
    source: &'static str,
    image: &'static str,
    paper: Option<&'static str>,
    video: Option<&'static str>,
}

#[function_component(Research)]
pub fn research(props: &ResearchData) -> Html {

    let image = html! { <cobul::Image size={ImageSize::Is3by2} src={props.image} /> };
    let footer = html! {
        <>
        <a href={props.paper}> {"Paper"} </a>
        <a href={props.video}> {"Video"} </a>
        </>
    };

    html! {
        <Card {image} {footer} fullheight=true>
            <Media>
                <p class="title is-4"> { props.name } </p>
                <p class="subtitle is-6"> { props.span } </p>
            </Media>

            <Content>
                <p> { props.description } </p>
                <i> {"Based on: "} {props.source} </i>
            </Content>
        </Card>
    }
}