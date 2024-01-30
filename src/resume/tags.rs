use cobul::*;
use implicit_clone::unsync::{IArray};
use yew::*;

use super::ResumeData;


#[derive(Properties, PartialEq, Clone, Debug)]
struct InnerProps {
    pub items: IArray<AttrValue>,
    pub title: AttrValue,
}

#[function_component(Inner)]
fn inner(props: &InnerProps) -> Html {
    let tag = |text| html! { <Tag size={Size::Large} color={Color::Light}> {text} </Tag> };

    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {props.title.clone()} </Title>
        <Tags> { for props.items.iter().map(tag) } </Tags>
        </>
    }
}

#[function_component(Hobbies)]
pub fn hobbies(resume: &ResumeData) -> Html {
    html!{ <Inner title="Hobbies" items={resume.hobbies.clone()} />}
}

#[function_component(Traits)]
pub fn traits(resume: &ResumeData) -> Html {
    html!{ <Inner title="Traits" items={resume.traits.clone()} />}
}


