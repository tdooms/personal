
mod awards;
mod education;
mod experience;
mod programming;
mod project;
mod tags;
mod skill;
mod language;

use implicit_clone::unsync::{IArray, IMap, IString};
use yew::*;
use cobul::{Column, Columns, Content, Block, ColumnSize};
use implicit_clone::ImplicitClone;
use yew::suspense::use_future;

pub use awards::*;
pub use education::*;
pub use experience::*;
pub use programming::*;
pub use project::*;
pub use tags::*;
pub use skill::*;
pub use language::*;
use crate::net::get_resume;
use crate::personalia::Personalia;

#[derive(serde::Deserialize, PartialEq, Clone)]
pub struct Cv {
    pub hobbies: IArray<IString>,
    pub skills: IMap<IString, IMap<IString, u32>>,
    pub traits: IArray<IString>,
    pub awards: IArray<IString>,
    pub languages: IArray<LanguageData>,

    pub programming: IArray<ProgrammingData>,
    pub education: IArray<EducationData>,
    pub experience: IArray<ExperienceData>,

    pub academic: IArray<ProjectData>,
    pub personal: IArray<ProjectData>,
}

impl ImplicitClone for Cv {}

#[derive(Properties, PartialEq, Clone)]
pub struct CvProps {
    pub cv: Cv,
}

#[function_component(Page)]
fn page(CvProps{cv}: &CvProps) -> Html {
    html! {
        <>
        <Columns>
            <Column>
            <Experience {cv} /> <Block />
            <Traits {cv} /> <Block />
            <Awards {cv} /> <Block />
            <Hobbies {cv} />
            </Column>
            <Column size={ColumnSize::Is4}>
            <Education {cv} /> <Block />
            <Programming {cv} />
            </Column>
        </Columns>
        <Columns>
            <Column> <Academic {cv} /> </Column>
            <Column> <Personal {cv} /> </Column>
        </Columns>
        </>
    }
}

#[function_component(Inner)]
fn inner() -> HtmlResult {
    let cv = (*use_future(get_resume)?).clone();
    let class = "pl-5 pt-0 mb-3 pb-0 has-background-light";

    let html = html! {
        <Columns>
        <Column size={ColumnSize::Is3} {class}>
            <Personalia />
            <Languages cv={cv.clone()} />
            <Skills cv={cv.clone()} />
        </Column>
        <Column>
            <Content> <Page cv={cv.clone()} /> </Content>
        </Column>
        </Columns>
    };
    Ok(html)
}

#[function_component(Resume)]
pub fn resume() -> Html {
    let fallback = html! {<div> {"Loading..."} </div>};
    html! { <Suspense {fallback}> <Inner /> </Suspense> }
}