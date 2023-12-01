use cobul::*;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct LanguageData {
    pub image: IString,
    pub title: IString,
    pub sub: IString,
}

impl ImplicitClone for LanguageData {}

#[function_component(Language)]
pub fn language(props: &LanguageData) -> Html {
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
