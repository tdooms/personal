use cobul::props::{ColumnSize};
use cobul::*;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct SkillData {
    pub name: &'static str,
    pub level: i64,
}

#[function_component(Skill)]
pub fn skill(props: &SkillData) -> Html {
    let f = |thresh| (thresh < props.level).then(|| "has-text-danger");

    html! {
        <Columns>
        <Column size={ColumnSize::Is7} class="py-1">
            <b>{props.name}</b>
        </Column>
        <Column class="py-1">
            {for (0..5).map(|i| html!{ <Icon icon={"fas fa-circle"} class={classes!(f(i))}/>}) }
        </Column>
        </Columns>
    }
}
