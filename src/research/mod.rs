mod topic;


use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ResearchData {
    pub name: AttrValue,
    pub description: AttrValue,
    pub kind: AttrValue,
    pub start: AttrValue,
    pub end: AttrValue,
    pub image: AttrValue,
    pub course: Option<AttrValue>,
    pub source: Option<AttrValue>,
    pub paper: Option<AttrValue>,
    pub presentation: Option<AttrValue>,
}


#[function_component(Research)]
pub fn research() -> yew::Html {
    html! {
        <div>
        {"Research"}
        </div>
    }
}