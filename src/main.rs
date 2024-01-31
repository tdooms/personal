mod nav;
mod net;
mod util;
mod personalia;
mod resume;
mod research;
mod blogs;
// mod christmas;
mod home;
mod page;
mod test;

use crate::nav::{Navbar, Route};
use crate::net::Remote;
use crate::resume::Resume;
use crate::test::Test;

use crate::blogs::{Blogs, Post};
use crate::home::Home;
use yew::*;
use yew_router::{BrowserRouter, Switch};

fn switch(route: Route) -> Html {
    let inner = match route.clone() {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <Remote<Blogs> path="static/blogs.json" /> },
        Route::Research => html! { <div> {"Research"} </div> },
        Route::Resume => html! { <Remote<Resume> path="static/resume.json" /> },
        Route::Post{name} => html! { <Post {name} /> },
        Route::Test => html! { <Test /> },
        // Route::Christmas => html! { <Christmas /> },
        // Route::Play{id} => html! { <Play id={id} /> },
    };
    html! {<> <Navbar {route} /> {inner}</> }
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
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}