mod nav;
mod net;
mod util;
mod personalia;
mod resume;
mod christmas;
mod home;

use crate::nav::{Navbar, Route};
use crate::resume::Resume;
use crate::christmas::{Christmas, Play};
use crate::home::Home;
use yew::*;
use yew_router::{BrowserRouter, Switch};


fn navbar(route: Route) -> Html {
    html! { <Navbar route={route} /> }
}

fn switch(route: Route) -> Html {
    let navbar = match route {
        Route::Play { .. } => html! { },
        _ => html! { <Navbar route={route.clone()} /> },
    };
    let inner = match route {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <div> {"Blog"} </div> },
        Route::Research => html! { <div> {"Research"} </div> },
        Route::Resume => html! { <Resume /> },
        Route::Christmas => html! { <Christmas /> },
        Route::Play{id} => html! { <Play id={id} /> },
    };
    html! {<>{navbar}{inner}</> }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    Renderer::<App>::new().render();
}