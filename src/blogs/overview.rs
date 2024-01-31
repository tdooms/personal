
use yew::*;
use cobul::*;
use cobul::Color::Success;
use cobul::ImageSize::Is3by2;
use yew_router::hooks::use_navigator;
use crate::{callback, nav::Route};

use super::{BlogData, BlogsData};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub blog: BlogData,
}

#[function_component(BlogCard)]
pub fn blog_card(props: &Props) -> Html {
    let BlogData { title, date, image, name } = props.blog.clone();
    

    let click = {
        let navigator = use_navigator().unwrap();
        callback!(move |_| navigator.push(&Route::Post{name: name.clone()}))
    };

    let footer = {
        let style = "border-top-left-radius: 0 !important;border-top-right-radius: 0 !important";
        html! { <Button {click} color={Success} {style} text={"Read"} fullwidth=true /> }
    };

    let image = {
        let src = image.clone();
        let style = "border-top-right-radius: 4px; border-top-left-radius: 4px;object-fit:cover";
        html! { <Image size={Is3by2} {src} {style} /> }
    };

    html! {
        <Column size={ColumnSize::Is4}>
        <Card {image} {footer}>
            <Title size={HeaderSize::Is5}> {title.clone()} </Title>
            <span> {date} </span>
        </Card>
        </Column>
    }
}

#[function_component(Overview)]
pub fn overview(blogs: &BlogsData) -> Html {
    html! {
        <Columns multiline=true>
             { for blogs.blogs.iter().map(|blog| html! { <BlogCard {blog} />})  }
        </Columns>
    }
}