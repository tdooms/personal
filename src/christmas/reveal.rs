
use std::f64::consts::PI;
use yew::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast};
use gloo::timers::callback::Timeout;

use std::rc::Rc;
use super::play::Images;
use super::vec2::Vec2;

fn dimensions() -> Vec2 {
    let window = web_sys::window().unwrap();

    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    Vec2::new(width, height)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct State {
    pub index: usize,
    pub radius: f64,
    pub centre: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    Radius,
    Centre,
}

impl Default for State {
    fn default() -> Self {
        Self { index: 0, radius: 0.0, centre: Vec2::between(Vec2::default(), dimensions()) }
    }
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let Self { radius, centre, index } = *self;

        let speed = 0.7; // amount of seconds for the full screen to be covered
        let increment = dimensions().max() / (60.0 * speed);


        match action {
            Action::Radius => Rc::new(Self { radius: radius + increment, centre, index }),
            Action::Centre => Rc::new(Self { index: index + 1, ..Default::default() }),
        }
    }
}


fn update_canvas(canvas: NodeRef, state: UseReducerHandle<State>, images: Images, done: Callback<()>) {
    let canvas = canvas.cast::<HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

    let Vec2 { x: width, y: height } = dimensions();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let State { radius, centre, index } = *state;

    context.save();
    context.arc(centre.x, centre.y, radius, 0.0, 2.0 * PI).unwrap();
    context.clip();

    let image = &images[index % images.len()];
    let dims = Vec2::new(image.width() as f64, image.height() as f64);

    let scale = (width / dims.x).min(height / dims.y);

    let x_offset = (width - dims.x * scale) / 2.0;
    let y_offset = (height - dims.y * scale) / 2.0;

    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&image, x_offset, y_offset, dims.x * scale, dims.y * scale);
    context.restore();

    state.dispatch(Action::Radius);

    if (dimensions() - centre).l2() < radius && centre.l2() < radius {
        state.dispatch(Action::Centre);
    }
    if index > (images.len() - 1) * 2 {
        done.emit(())
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub images: Images,
    pub done: Callback<()>,
}

#[function_component(Reveal)]
pub fn reveal(props: &Props) -> Html {
    let Props { images, done } = props.clone();

    let canvas_ref = use_node_ref();
    let state = use_reducer(State::default);

    let canvas = canvas_ref.clone();

    let _timer = use_memo(state, move |state| {
        let state = state.clone();
        let images = images.clone();
        let done = done.clone();
        Timeout::new(16, || update_canvas(canvas, state, images, done))
    });

    html! {
        <>
        <canvas style="display:block" ref={canvas_ref} />
        </>
    }
}