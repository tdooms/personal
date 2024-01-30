use yew::*;
use cobul::*;
use crate::personalia::Personalia;
use crate::pane::Pane;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Columns>
        <Pane > <Personalia /> </Pane>
        <Column style="height: 96vh"> <Container> {"Under construction"} </Container> </Column>
        </Columns>
    }
}