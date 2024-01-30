



use yew::*;
use yew::suspense::{use_future, UseFutureHandle};

use gloo::net::http::Request;
use serde::de::DeserializeOwned;

pub async fn fetch<T: DeserializeOwned>(path: AttrValue) -> anyhow::Result<T> {
    Ok(Request::get(path.as_str())
        .send()
        .await?
        .json::<T>()
        .await?)
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub path: AttrValue
}

#[function_component(RemoteInner)]
fn remote_inner<C: BaseComponent>(props: &Props) -> HtmlResult where C::Properties: Clone + DeserializeOwned{
    let handle: UseFutureHandle<anyhow::Result<C::Properties>> = use_future(|| fetch(props.path.clone()))?;
    let data: C::Properties = handle.as_ref().unwrap().clone();

    Ok(html! { <C ..data />})
}

#[function_component(Remote)]
pub fn remote<C: BaseComponent>(props: &Props) -> Html where C::Properties: Clone + DeserializeOwned {
    let fallback = html! {};
    yew::html!{
        <Suspense {fallback}> <RemoteInner<C> ..props.clone()/> </Suspense>
    }
}