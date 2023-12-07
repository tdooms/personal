mod achievement;
mod education;
mod experience;
mod language;
mod project;
mod skill;
mod r#trait;
mod research;
mod contact;
mod image;
mod profile;
mod util;

use achievement::*;
use education::*;
use experience::*;
use language::*;
use project::*;
use skill::*;
use r#trait::*;
use research::*;
use contact::*;
use profile::*;

use cobul::*;
use cobul::icons::{Brands, Solid};
use yew::prelude::*;
use yew_router::BrowserRouter;
use strum::{EnumIter, Display};
use implicit_clone::unsync::{IArray, IMap, IString};


#[derive(serde::Deserialize)]
struct Cv {
    profile: ProfileData,
    introduction: IArray<IString>,
    contact: ContactData,

    hobbies: IArray<IString>,
    skills: IMap<IString, IMap<IString, u32>>,
    traits: IArray<IString>,
    achievements: IArray<IString>,

    educations: IArray<EducationData>,
    languages: IArray<LanguageData>,

    // trajectory: IArray<IString>,
    experiences: IArray<ExperienceData>,
    explanation: IArray<IString>,

    evolution: IArray<IString>,
    academic: IArray<ProjectData>,
    personal: IArray<ProjectData>,

    research: IArray<ResearchData>
}

#[derive(Clone, Copy, EnumIter, PartialEq, Display)]
enum Tab {
    General,
    Experience,
    Projects,
    Research,
}

impl Tab {
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

    let general = html! {
        <Content>
        <Title size={HeaderSize::Is4} class="mb-3"> {"Introduction"} </Title>
        { for cv.introduction.iter().map(|intro| html!{ <p style="text-align: justify">{intro}</p> }) }

        <Block />

        <Title size={HeaderSize::Is4} class="mb-3"> {"Traits"} </Title>
        <Tags> { for cv.traits.iter().map(|x| html! {<Trait name={x}/>}) } </Tags>

        <Block />

        <Title size={HeaderSize::Is4} class="mb-3"> {"Achievements, Honours and Awards"} </Title>
        { for cv.achievements.iter().map(|x| html! {<Achievement text={x}/>}) }

        <Block />

        <Title size={HeaderSize::Is4} class="mb-3"> {"Hobbies & interests"} </Title>
        <Tags> { for cv.hobbies.iter().map(|x| html! {<Tag size={Size::Large}> {x} </Tag>}) } </Tags>

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
        <Block style="margin-bottom: 2.9rem" />
        </>
    };

    let evolution = html! {
        { for cv.evolution.iter().map(|x| html! {<Block style="text-align: center">{x}</Block>}) }
    };

    let explanation = html! {
        { for cv.explanation.iter().map(|x| html! {<Block style="text-align: center">{x}</Block>}) }
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
        { for cv.research.iter().map(|x| html! {<Research ..x.clone()/>}) }
    };

    let class = "pl-5 pt-0 mb-3 pb-0 has-background-light";

    let view_tab = |tab: Tab| {
        let state = if tab == model.value() {"my-navbar-selected"} else {"my-navbar-item"};
        let class = classes!("column", state);

        html! {
            <div {class} onclick={model.reform(move |_| tab)}>
            <Icon icon={tab.icon().unwrap()} /> <span> {tab.to_string()} </span>
            </div>
        }
    };

    let body = match model.value() {
        Tab::General => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class} style="height: 100vh">
                <Profile ..cv.profile />
                <Contact ..cv.contact />
                <Block class="my-6"/>
                {languages}
            </Column>
            <Column class="mx-4"> {general} </Column>
            <Column class="mr-4" size={ColumnSize::Is3}> {education} </Column>
            </Columns>
        },
        Tab::Experience => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class} style="height: 100vh">
                <Profile ..cv.profile />
                { for cv.skills.iter().map(|(k, v)| html! { <Skills field={k} skills={v} />}) }
            </Column>
            <Column class="mx-4"> {experience} </Column>
            <Column class="mr-4" size={ColumnSize::Is3}> {prog_languages} </Column>
            </Columns>
        },
        Tab::Projects => html! {
            <Columns>
            <Column size={ColumnSize::Is3} {class} style="height: 100vh">
                <Profile ..cv.profile />
                {evolution}
            </Column>
            <Column class="mx-4"> {academic} </Column>
            <Column class="mr-4"> {personal} </Column>
            </Columns>
        },
        Tab::Research => html! {
            <Columns>
            <Column size={ColumnSize::Is3} style="height: 100vh" {class}>
                <Profile ..cv.profile />
                {explanation}
            </Column>
            <Column class="mx-4"> {research} </Column>
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
