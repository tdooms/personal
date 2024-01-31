use yew::*;
use cobul::*;
use crate::personalia::Personalia;
use crate::page::Page;

#[function_component(Home)]
pub fn home() -> Html {
    let pane = html! {<Personalia />};


    html! { 
        <Page {pane} fullheight=true> 
            <Title size={HeaderSize::Is4}> {"Hi! I am Thomas."}</Title>
            <Subtitle size={HeaderSize::Is5}> {"I'm a machine learning researcher interested in understanding how neural networks work."}</Subtitle>

            
        </Page> 
    }
}