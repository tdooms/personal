use cobul::props::{ColumnSize};
use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct LanguageData {
    pub image: &'static str,
    pub title: &'static str,
    pub sub: &'static str,
}

#[function_component(Language)]
pub fn language(props: &LanguageData) -> Html {
    html! {
        <Column size={ColumnSize::Is6}>
        <Card>
            <Block>
            <img style="height:70px; margin-left:auto; margin-right:auto; display: block;" src={props.image}/>
            </Block>

            <p class="has-text-centered"><b> {props.title} </b></p>
            <p class="has-text-grey has-text-centered"> {props.sub} </p>
        </Card>
        </Column>
    }
}
