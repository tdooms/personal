use cobul::icons::Solid;
use cobul::*;

use strum::{Display, EnumIter};
use yew::{classes, function_component, html, AttrValue, Callback, Html, Properties};
use yew_router::hooks::use_navigator;
use yew_router::Routable;

#[derive(Clone, EnumIter, PartialEq, Display, Routable)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/research")]
    Research,
    #[at("/resume")]
    Resume,
    // #[at("/christmas")]
    // Christmas,
    // #[at("/christmas/play/:id")]
    // Play { id: String },
}

impl Route {
    fn icon(&self) -> AttrValue {
        match self {
            Self::Home => Solid::House,
            Self::Blog => Solid::Fire,
            Self::Resume => Solid::List,
            Self::Research => Solid::Atom,
            // Self::Christmas => Solid::Gifts,
            // Self::Play { .. } => Solid::Play,
        }.to_string().into()
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub route: Route,
}

#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let route = props.route.clone();

    let navigator = use_navigator().unwrap();
    let goto = Callback::from(move |route: Route| navigator.push(&route));

    let view_tab = |tab: Route| {
        let state = if tab == route {"my-navbar-selected"} else {"my-navbar-item"};
        let class = classes!("column", state);

        let onclick = {
            let tab = tab.clone();
            goto.reform(move |_| tab.clone())
        };

        html! {
            <div {class} {onclick}>
            <Icon icon={tab.icon()} /> <span> {tab.to_string()} </span>
            </div>
        }
    };

    // let onclick = goto.reform(move |_| Route::Christmas);

    html! {
        <Columns class="has-background-light pt-3 has-text-centered">
        <Column size={ColumnSize::Is3} />
        {view_tab(Route::Home)}
        {view_tab(Route::Blog)}
        {view_tab(Route::Research)}
        {view_tab(Route::Resume)}
        <Column size={ColumnSize::Is3} />
        // <Column class="pt-1 pb-0">
        // // <a {onclick}> <img src="static/christmas/tree.png" width=36 height=36 /> </a>
        // </Column>
        </Columns>
    }
}
