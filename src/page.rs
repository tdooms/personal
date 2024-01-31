use yew::*;
use cobul::*;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub pane: Option<Html>,

    #[prop_or_default]
    pub fullheight: bool,
}

#[function_component(Page)]
pub fn page(props: &Props) -> Html {
    let class = "pl-5 pt-0 mb-3 pb-0 has-background-light";
    let style = if props.fullheight {"height: 96vh" } else {""};

    match props.pane.clone() {
        Some(pane) => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class}> {pane} </Column> 
            <Column> <Container {style}> {props.children.clone()} </Container> </Column>
            </Columns>
        },
        None => html! {
            <Container {style}> {props.children.clone()}</Container>
        },
    }
}
