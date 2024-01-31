use cobul::*;
use yew::*;
use implicit_clone::ImplicitClone;

use super::ResumeData;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ProgrammingData {
    pub image: AttrValue,
    pub title: AttrValue,
    pub sub: AttrValue,
}

impl ImplicitClone for ProgrammingData {}

#[function_component(Item)]
fn item(props: &ProgrammingData) -> Html {
    html! {
        <Column size={ColumnSize::Is6}>
        <Card>
            <Block>
            <img style="height:51px; margin-left:auto; margin-right:auto; display: block;" src={props.image.clone()}/>
            </Block>

            <p class="has-text-centered"><b> {props.title.clone()} </b></p>
            <p class="has-text-grey has-text-centered"> {props.sub.clone()} </p>
        </Card>
        </Column>
    }
}

#[function_component(Programming)]
pub fn programming(resume: &ResumeData) -> Html {
    html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Programming"} </Title>
        <Columns multiline=true>
        { for resume.programming.iter().map(|data| html! {<Item ..data />}) }
        </Columns>
        </>
    }
}



