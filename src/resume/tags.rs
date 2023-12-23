use cobul::*;
use implicit_clone::unsync::{IArray, IString};
use yew::*;
use crate::resume::CvProps;


#[derive(Properties, PartialEq, Clone, Debug)]
struct Props {
    pub items: IArray<IString>,
    pub title: IString,
}

#[function_component(Inner)]
fn inner(props: &Props) -> Html {
    let tag = |text| html! { <Tag size={Size::Large} color={Color::Light}> {text} </Tag> };

    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {props.title.clone()} </Title>
        <Tags>
        { for props.items.iter().map(tag) }
        </Tags>
        </>
    }
}

#[function_component(Hobbies)]
pub fn hobbies(CvProps{cv}: &CvProps) -> Html {
    html!{ <Inner title="Hobbies" items={cv.hobbies.clone()} />}
}

#[function_component(Traits)]
pub fn traits(CvProps{cv}: &CvProps) -> Html {
    html!{ <Inner title="Traits" items={cv.traits.clone()} />}
}


