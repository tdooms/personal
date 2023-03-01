mod achievement;
mod education;
mod experience;
mod language;
mod project;
mod skill;
mod talent;
mod research;
mod contact;

use achievement::*;
use education::*;
use experience::*;
use language::*;
use project::*;
use skill::*;
use talent::*;
use research::*;
use contact::*;

use cobul::*;
use std::collections::HashMap;
use cobul::fa::{Brands, Solid};
use cobul::simple::HasIcon;
use yew::prelude::*;
use yew_router::BrowserRouter;
use strum::{EnumIter, Display};
use crate::contact::ContactData;


#[derive(serde::Deserialize)]
struct Cv {
    name: &'static str,
    location: &'static str,
    profession: &'static str,
    introduction: Vec<&'static str>,

    contact: ContactData,
    hobbies: Vec<&'static str>,
    skills: HashMap<&'static str, HashMap<&'static str, i64>>,
    talents: Vec<&'static str>,
    achievements: Vec<&'static str>,

    educations: Vec<EducationData>,
    languages: Vec<LanguageData>,

    trajectory: Vec<&'static str>,
    experiences: Vec<ExperienceData>,

    evolution: Vec<&'static str>,
    academic: Vec<ProjectData>,
    personal: Vec<ProjectData>,

    research: Vec<ResearchData>
}

#[derive(Clone, Copy, EnumIter, PartialEq, Display)]
enum Tab {
    General,
    Experience,
    Projects,
    Research,
}

impl HasIcon for Tab {
    fn icon(&self) -> Option<String> {
        match self {
            Tab::General => Solid::List.to_string(),
            Tab::Experience => Solid::Fire.to_string(),
            Tab::Projects => Brands::Github.to_string(),
            Tab::Research => Solid::Atom.to_string(),
        }.into()
    }
}

#[function_component(App)]
fn app() -> Html {
    let json = include_str!("../cv.json");
    let cv: Cv = serde_json::from_str(json).unwrap();
    let model = use_model(|| Tab::General);

    let view_skills = |(key, values): (&&'static str, &HashMap<&'static str, i64>)| {
        html! {
            <>
            <Subtitle class="has-text-danger"> {key} </Subtitle>
            { for values.iter().map(|(name, &level)| html! {<Skill ..SkillData{name, level}/>}) }
            </>
        }
    };

    let profile = html! {
        <>
        <div class="has-text-centered">
        <Image size={ImageSize::Is128x128} rounded=true src="picture.jpg" class="is-inline-block"/>
        </div>

        <Subtitle size={HeaderSize::Is4} class="has-text-centered mb-3"> {cv.name} </Subtitle>
        <Subtitle size={HeaderSize::Is6} class="has-text-centered has-text-grey"> {cv.location} </Subtitle>

        <Block>
        <Title size={HeaderSize::Is5} class="has-text-centered"> {cv.profession} </Title>
        </Block>

        <hr class="has-background-black"/>
        </>
    };

    let skills = html! {
        { for cv.skills.iter().map(view_skills) }
    };

    let general = html! {
        <Content>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Introduction"} </Title>
        { for cv.introduction.iter().map(|intro| html!{ <p style="text-align: justify">{intro}</p> }) }

        <Title size={HeaderSize::Is4} class="mb-3"> {"Talents"} </Title>

        <Tags> { for cv.talents.iter().map(|x| html! {<Talent name={x.clone()}/>}) } </Tags>

        <Title size={HeaderSize::Is4} class="mb-3"> {"Achievements, Honours and Awards"} </Title>
        { for cv.achievements.iter().map(|x| html! {<Achievement text={x.clone()}/>}) }

        <Title size={HeaderSize::Is4} class="mb-3"> {"Hobbies & interests"} </Title>

        <Tags> { for cv.hobbies.iter().map(|x| html! {<Tag size={Size::Large}> {x.clone()} </Tag>}) } </Tags>

        </Content>
    };

    let languages = html! {
        <>
        <Columns class="mb-1 mt-4">
            <Column class="has-text-left py-0"> <strong> {"Dutch"} </strong> </Column>
            <Column class="has-text-right py-0"> {"native"} </Column>
        </Columns>
        <progress class="progress is-danger" style="height:0.5rem" value="100" max="100">{"100%"}</progress>

        <Columns class="mb-1 mt-4">
            <Column class="has-text-left py-0"> <strong> {"French"} </strong> </Column>
            <Column class="has-text-right py-0"> {"bilingual"} </Column>
        </Columns>
        <progress class="progress is-danger" style="height:0.5rem" value="90" max="100">{"90%"}</progress>

        <Columns class="mb-1 mt-4">
            <Column class="has-text-left py-0"> <strong> {"English"} </strong> </Column>
            <Column class="has-text-right py-0"> {"proficient"} </Column>
        </Columns>
        <progress class="progress is-danger" style="height:0.5rem" value="80" max="100">{"80%"}</progress>
        </>
    };

    let trajectory = html! {
        { for cv.trajectory.iter().map(|x| html! {<Block style="text-align: center">{x}</Block>}) }
    };

    let evolution = html! {
        { for cv.evolution.iter().map(|x| html! {<Block style="text-align: center">{x}</Block>}) }
    };

    let experience = html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Experience"} </Title>
        { for cv.experiences.iter().map(|x| html! {<Experience ..x.clone()/>}) }
        </>
    };

    let academic = html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Academic projects"} </Title>
        { for cv.academic.iter().map(|x| html! {<Project ..x.clone()/>}) }
        </>
    };

    let personal = html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Personal projects"} </Title>
        { for cv.personal.iter().map(|x| html! {<Project ..x.clone()/>}) }
        </>
    };

    let prog_languages = html! {
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Languages"} </Title>
        <Columns multiline=true> { for cv.languages.iter().map(|x| html! {<Language ..x.clone()/>}) } </Columns>
        </>
    };

    let education = html!{
        <>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Education"} </Title>
        { for cv.educations.iter().map(|x| html! {<Education ..x.clone()/>}) }
        </>
    };

    let research = html! {
        <Columns>
        { for cv.research.iter().map(|x| html! {<Column> <Research ..x.clone()/> </Column>}) }
        </Columns>
    };

    let class = "pl-5 pt-0 mb-3 pb-0 has-background-light";

    let options = ["hoverable", "selected"];

    let view_tab = |tab: Tab| {
        let state = if tab == model.value {"selected"} else {"hoverable"};
        let class = classes!("column", state);

        html! {
            <div {class} onclick={model.input.reform(move |_| tab)} style="cursor:pointer">
            <Icon icon={tab.icon().unwrap()} /> <span> {tab.to_string()} </span>
            </div>
        }
    };

    let body = match model.value {
        Tab::General => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class}> {profile} <Contact ..cv.contact /> <hr class="has-background-black"/> {languages} </Column>
            <Column class="mx-4"> {general} </Column>
            <Column class="mr-4" size={ColumnSize::Is3}> {education} </Column>
            </Columns>
        },
        Tab::Experience => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class}> {profile} {skills} </Column>
            <Column class="mx-4"> {experience} </Column>
            <Column class="mr-4" size={ColumnSize::Is3}> {prog_languages} </Column>
            </Columns>
        },
        Tab::Projects => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class}> {profile} {evolution} </Column>
            <Column class="mx-4"> {academic} </Column>
            <Column class="mr-4"> {personal} </Column>
            </Columns>
        },
        Tab::Research => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class}> {profile} {"heya"} </Column>
            <Column class="px-0"> {research} </Column>
            </Columns>
        }
    };

    html! {
        <BrowserRouter>
        <Columns class="has-background-light pt-3 has-text-centered">
        <Column size={ColumnSize::Is3} />
        {view_tab(Tab::General)}
        {view_tab(Tab::Experience)}
        {view_tab(Tab::Projects)}
        {view_tab(Tab::Research)}
        <Column size={ColumnSize::Is3} />
        </Columns>

        {body}
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
