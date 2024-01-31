pub use medicine::*;
use implicit_clone::{sync::IArray, ImplicitClone};
pub use overview::*;

use cobul::*;
use serde::Deserialize;
use yew::*;
use crate::page::{Page};

mod medicine;
mod overview;
mod footnote;

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogData {
    pub title: AttrValue,
    pub date: AttrValue,
    pub image: AttrValue,
    pub name: AttrValue,
}

impl ImplicitClone for BlogData {}

#[derive(Clone, PartialEq, Properties, Deserialize)]
pub struct BlogsData {
    pub blogs: IArray<BlogData>
}


#[function_component(Blogs)]
pub fn blogs(blogs: &BlogsData) -> Html {
    log::info!("blogs: {:?}", blogs.blogs.len());
    html! { <Page pane={html!{}} fullheight=true> <Overview ..blogs.clone() /> </Page> }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    match props.name.as_str() {
        "medicine" => html! { <Page> <Medicine /> </Page> },
        _ => html! { <Page> {"not found"} </Page> },
    }
}