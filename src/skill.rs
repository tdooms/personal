use implicit_clone::unsync::{IMap, IString};
use cobul::*;
use yew::*;

#[derive(Properties, PartialEq, Clone, Debug)]
struct SkillData {
    pub name: IString,
    pub level: u32,
}

#[function_component(Skill)]
fn skill(props: &SkillData) -> Html {
    let class = |thresh| classes!((thresh < props.level).then(|| "has-text-danger"));

    html! {
        <Columns>
        <Column size={ColumnSize::Is7} class="py-1 pr-0">
            <b>{props.name.clone()}</b>
        </Column>
        <Column class="py-1 pl-0">
            {for (0..5).map(|i| html!{ <Icon icon={"fas fa-circle"} class={class(i)}/>}) }
        </Column>
        </Columns>
    }
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct SkillsData {
    pub skills: IMap<IString, u32>,
    pub field: IString
}

#[function_component(Skills)]
pub fn skills(props: &SkillsData) -> Html {
    html! {
        <>
        <Subtitle class="has-text-danger"> {props.field.clone()} </Subtitle>
        { for props.skills.iter().map(|(name, level)| html! {<Skill {name} {level} />}) }
        </>
    }
}
