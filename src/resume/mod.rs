
mod awards;
mod education;
mod experience;
mod programming;
mod project;
mod tags;
mod skill;
mod language;

use implicit_clone::unsync::{IArray, IMap};
use yew::*;
use cobul::{Column, Columns, Content, Block, ColumnSize};
use implicit_clone::ImplicitClone;


pub use awards::*;
pub use education::*;
pub use experience::*;
pub use programming::*;
pub use project::*;
pub use tags::*;
pub use skill::*;
pub use language::*;
use crate::{page::Page, personalia::Personalia};

#[derive(Properties, serde::Deserialize, PartialEq, Clone)]
pub struct ResumeData {
    pub hobbies: IArray<AttrValue>,
    pub skills: IMap<AttrValue, IMap<AttrValue, u32>>,
    pub traits: IArray<AttrValue>,
    pub awards: IArray<AttrValue>,
    pub languages: IArray<LanguageData>,

    pub programming: IArray<ProgrammingData>,
    pub education: IArray<EducationData>,
    pub experience: IArray<ExperienceData>,

    pub academic: IArray<ProjectData>,
    pub personal: IArray<ProjectData>,
}

impl ImplicitClone for ResumeData {}

#[function_component(Inner)]
fn inner(resume: &ResumeData) -> Html {
    html! {
        <>
        <Columns>
            <Column>
            <Experience ..resume.clone() /> <Block />
            <Traits ..resume.clone() /> <Block />
            <Awards ..resume.clone() /> <Block />
            <Hobbies ..resume.clone() />
            </Column>
            <Column size={ColumnSize::Is4}>
            <Education ..resume.clone() /> <Block />
            <Programming ..resume.clone() />
            </Column>
        </Columns>
        <Columns>
            <Column> <Academic ..resume.clone() /> </Column>
            <Column> <Personal ..resume.clone() /> </Column>
        </Columns>
        </>
    }
}

#[function_component(Resume)]
pub fn resume(resume: &ResumeData) -> Html {
    let pane = html! {
        <>
        <Personalia />
        <Languages ..resume.clone() />
        <Skills ..resume.clone() />
        </>
    };

    html! {<Page {pane}> <Inner ..resume.clone() /> </Page> }
}