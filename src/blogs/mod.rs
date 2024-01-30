pub use blog::*;
use implicit_clone::{sync::IArray, ImplicitClone};
pub use overview::*;

use cobul::*;
use serde::Deserialize;
use yew::*;
use crate::pane::Pane;

mod blog;
mod overview;

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogData {
    pub title: AttrValue,
    pub date: AttrValue,
    pub image: AttrValue,
}

impl ImplicitClone for BlogData {}

#[derive(Clone, PartialEq, Properties, Deserialize)]
pub struct BlogsData {
    pub blogs: IArray<BlogData>
}


#[function_component(Blogs)]
pub fn blogs(blogs: &BlogsData) -> Html {
    html! {
        <Columns>
        <Pane>
            <Overview />
        </Pane>
        <Column>
            <Blog />
        </Column>
        </Columns>
    }
}