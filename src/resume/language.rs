use cobul::*;
use yew::*;
use implicit_clone::{ImplicitClone, unsync::IString};
use crate::resume::CvProps;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct LanguageData {
    pub name: IString,
    pub level: IString,
    pub value: u64,
}

impl ImplicitClone for LanguageData {}

#[function_component(Language)]
pub fn language(props: &LanguageData) -> Html {
    html! {
        <>
        <Columns class="mb-1 mt-4">
            <Column class="has-text-left py-0"> <strong> {props.name.clone()} </strong> </Column>
            <Column class="has-text-right py-0"> {props.level.clone()} </Column>
        </Columns>
        <progress class="progress is-danger" style="height:0.5rem" value={props.value.to_string()} max="100">{props.value}{"%"}</progress>
        </>
    }
}

#[function_component(Languages)]
pub fn languages(CvProps{cv}: &CvProps) -> Html {
    html! {for cv.languages.iter().map(|data| html! {<Language ..data />})}
}
