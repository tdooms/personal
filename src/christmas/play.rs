use std::collections::HashMap;
use std::rc::Rc;

use web_sys::HtmlImageElement;
use yew::*;
use yew_router::hooks::use_navigator;
use crate::nav::Route;

use super::start::Start;
use super::reveal::Reveal;
use super::wait::Wait;
use super::summary::Summary;

pub type Quiz = Rc<HashMap<AttrValue, AttrValue>>;
pub type Data = Rc<HashMap<AttrValue, Quiz>>;
pub type Images = Rc<Vec<HtmlImageElement>>;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: AttrValue,
}

enum Stage {
    Start,
    Reveal,
    Wait,
    Summary,
}

#[function_component(Play)]
pub fn play(props: &Props) -> Html {
    let stage = use_state(|| Stage::Start);

    let data = use_memo((), |_| {
        let str = include_str!("data.json");
        serde_json::from_str::<Data>(str).unwrap()
    });

    let convert = |(_, v): (&AttrValue, &AttrValue)| {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(v.as_str());
        image
    };

    let images: Vec<_> = data.get(&props.id).unwrap().iter().map(convert).collect();
    let images = Rc::new(images);

    let names = data.get(&props.id).unwrap().iter().map(|(a, b)| (a.clone(), b.clone())).collect::<Vec<_>>();

    let play = {
        let cloned = stage.clone();
        Callback::from(move |_| cloned.set(Stage::Reveal))
    };
    let wait = {
        let cloned = stage.clone();
        Callback::from(move |_| cloned.set(Stage::Wait))
    };
    let summary = {
        let cloned = stage.clone();
        Callback::from(move |_| cloned.set(Stage::Summary))
    };
    let leave = {
        let nav = use_navigator().unwrap();
        Callback::from(move |_| nav.push(&Route::Christmas))
    };

    match *stage {
        Stage::Start => html! { <Start done={play} /> },
        Stage::Reveal => html! { <Reveal {images} done={wait} /> },
        Stage::Wait => html! { <Wait done={summary} /> },
        Stage::Summary => html! { <Summary done={leave} images={names} />},
    }
}