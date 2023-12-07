use implicit_clone::unsync::IString;
use yew::Callback;

pub fn redirect(url: IString) -> Callback<()> {
    Callback::from(move |_| {
        web_sys::window().unwrap().open_with_url(&url).unwrap();
    })
}