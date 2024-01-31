
use yew::*;

pub fn redirect(url: AttrValue) -> Callback<()> {
    Callback::from(move |_| {
        web_sys::window().unwrap().open_with_url(&url).unwrap();
    })
}

#[macro_export]
macro_rules! callback {
    ( $y:expr ) => {
        yew::Callback::from($y)
    };
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            yew::Callback::from($y)
        }
    };
}