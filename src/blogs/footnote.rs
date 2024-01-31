use crate::callback;
use yew::*;
use cobul::*;
use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

#[derive(Clone)]
pub struct FootnoteContext {
    counter: Rc<AtomicUsize>
}

impl PartialEq for FootnoteContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.counter, &other.counter)
    }
}

#[hook]
pub fn use_footnote_context(max: usize) -> FootnoteContext {
    let counter = Rc::new(AtomicUsize::new(max));
    FootnoteContext { counter }
}



#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: Html,
    pub ctx: FootnoteContext
}

#[function_component(Footnote)]
pub fn footnote(props: &Props) -> Html {
    let counter = props.ctx.counter.clone();
    let value = use_state(move || counter.fetch_sub(1, Ordering::Relaxed));
    let hovered = use_state(|| false);

    let onmouseover = callback!(hovered; move |_| hovered.set(true));
    let onmouseout = callback!(hovered; move |_| hovered.set(false));

    let preview = match *hovered {
        true => html! {<Box style="position:absolute;width:700px"> {props.text.clone()} </Box>},
        false => html! {},
    };

    html! {
        <>
        <sup {onmouseover} {onmouseout}> <a>{" ["}{*value}{"]"}</a> </sup>
        {preview}
        </>
    }
}