use yew::*;
use cobul::*;
use crate::personalia::Personalia;

#[function_component(Home)]
pub fn home() -> Html {
    let class = "pl-5 pt-0 mb-3 pb-0 has-background-light";

    html! {
        <Columns>
        <Column size={ColumnSize::Is3} {class}> <Personalia /> </Column>
        <Column style="height: 96vh"> <Container> {"Under construction"} </Container> </Column>
        </Columns>
    }
}