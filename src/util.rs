
use yew::*;

pub fn redirect(url: AttrValue) -> Callback<()> {
    Callback::from(move |_| {
        web_sys::window().unwrap().open_with_url(&url).unwrap();
    })
}

