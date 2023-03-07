use yew::prelude::*;
use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Height {
    Vh(u32),
    Px(u32),
}

impl Default for Height {
    fn default() -> Self {
        Height::Vh(100)
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub src: Option<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub height: Height,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: String,

    #[prop_or_default]
    pub fit: Fit,

    #[prop_or_default]
    pub border: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Display)]
pub enum Fit {
    #[strum(serialize = "none")]
    None,
    #[default]
    #[strum(serialize = "contain")]
    Contain,
    #[strum(serialize = "cover")]
    Cover,
    #[strum(serialize = "scale-down")]
    ScaleDown,
    #[strum(serialize = "fill")]
    Fill,
}

#[function_component(DynImage)]
pub fn dyn_image(props: &Props) -> Html {
    let height = match props.height {
        Height::Vh(vh) => format!("height:{}vh", vh),
        Height::Px(px) => format!("height:{}px", px),
    };

    let border = "border-width:thin;border-style:solid;border-radius:5px;border-color:lightgray";
    let fit = format!("object-fit:{};display:block;margin-left:auto;margin-right:auto", props.fit);
    let style = format!("{height};{fit};{}", props.border.then(|| border).unwrap_or_default());

    html! {
        <div style="justify-content:center" class="p-0 m-0 is-flex">
            <img src={ props.src.clone() } {style}/>
        </div>
    }
}