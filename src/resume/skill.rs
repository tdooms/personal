use implicit_clone::unsync::{IMap, IString};
use cobul::*;
use yew::*;
use crate::resume::CvProps;

#[derive(Properties, PartialEq, Clone, Debug)]
struct SkillData {
    pub name: IString,
    pub level: u32,
}

#[derive(Properties, PartialEq, Clone, Debug)]
struct FieldData {
    pub skills: IMap<IString, u32>,
    pub field: IString
}

#[function_component(Item)]
fn item(props: &SkillData) -> Html {
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

#[function_component(Field)]
fn field(props: &FieldData) -> Html {
    html! {
        <>
        <Subtitle class="has-text-danger"> {props.field.clone()} </Subtitle>
        { for props.skills.iter().map(|(name, level)| html! {<Item {name} {level} />}) }
        </>
    }
}

#[function_component(Skills)]
pub fn skills(CvProps{cv}: &CvProps) -> Html {
    html! {
        <>
        // <Title size={HeaderSize::Is4} class="mb-3"> {"Skills"} </Title>
        { for cv.skills.iter().map(|(field, skills)| html! {<Field {field} {skills} />}) }
        </>
    }
}
