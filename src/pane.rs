use yew::*;
use cobul::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Pane)]
pub fn pane(props: &Props) -> Html {
    let class = "pl-5 pt-0 mb-3 pb-0 has-background-light";

    html! {
        <Column size={ColumnSize::Is3} {class}>
            {for props.children.iter()}
        </Column>
    }
}