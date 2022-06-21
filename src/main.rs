mod achievement;
mod education;
mod experience;
mod language;
mod skill;
mod talent;

use achievement::*;
use education::*;
use experience::*;
use language::*;
use skill::*;
use talent::*;

use cobul::props::{Color, ColumnSize, HeaderSize, ImageSize};
use cobul::*;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(serde::Deserialize)]
struct Cv {
    name: &'static str,
    location: &'static str,
    profession: &'static str,
    introduction: &'static str,

    skills: HashMap<&'static str, HashMap<&'static str, i64>>,
    talents: Vec<&'static str>,
    achievements: Vec<&'static str>,

    educations: Vec<EducationData>,

    languages: Vec<LanguageData>,
    experiences: Vec<ExperienceData>,
}

#[function_component(App)]
fn app() -> Html {
    let json = include_str!("../cv.json");
    let cv: Cv = serde_json::from_str(json).unwrap();

    let view_skills = |(key, values): (&&'static str, &HashMap<&'static str, i64>)| {
        html! {
            <>
            <Subtitle class="has-text-danger"> {key} </Subtitle>
            { for values.iter().map(|(name, &level)| html! {<Skill ..SkillData{name, level}/>}) }
            </>
        }
    };

    let skills = html! {
        { for cv.skills.iter().map(view_skills) }
    };

    let talents = html! {
        { for cv.talents.iter().map(|x| html! {<Talent name={x.clone()}/>}) }
    };

    let frameworks = html! {
        { for cv.languages.iter().map(|x| html! {<Language ..x.clone()/>}) }
    };

    let educations = html! {
        { for cv.educations.iter().map(|x| html! {<Education ..x.clone()/>}) }
    };

    let achievements = html! {
        { for cv.achievements.iter().map(|x| html! {<Achievement text={x.clone()}/>}) }
    };

    let left = html! {
        <>
            <Columns centered=true>
            <Column size={ColumnSize::Is6}>
            <Image size={ImageSize::Is128x128} rounded=true src="picture.jpg"/>
            </Column>
            </Columns>


            <Subtitle size={HeaderSize::Is4} class="has-text-centered mb-3"> {"Thomas Dooms"} </Subtitle>
            <Subtitle size={HeaderSize::Is6} class="has-text-centered has-text-grey"> {"Antwerp, Belgium"} </Subtitle>

            <Block>
            <Title size={HeaderSize::Is5} class="has-text-centered"> {"Data scientist"} </Title>
            </Block>

            <hr class="has-background-black"/>
            {skills}
        </>
    };

    let center = html! {
        <Content>
        <Title> {"Introduction"} </Title>
        <p>{cv.introduction}</p>
        <Title> {"Talents"} </Title>
        <Tags>
        {talents}
        </Tags>
        <Title> {"Academic projects"} </Title>
        <Title> {"Personal projects"} </Title>
        </Content>
    };

    let right = html! {
        <>
        <Title> {"Languages"} </Title>
        <Columns multiline=true>
        {frameworks}
        </Columns>
        <Title> {"Education"} </Title>
        {educations}
        </>
    };

    let navbar = html! {
        <Navbar onclick={Callback::noop()}> </Navbar>
    };

    html! {
        <>
        {navbar}

        <Columns>
        <Column size={ColumnSize::Is3} class="m-4 pt-6 has-background-light">
            {left}
        </Column>
        <Column class="m-4 pt-6" >
            {center}
        </Column>
        <Column class="m-4 pt-6" size={ColumnSize::Is3}>
            {right}
        </Column>
        </Columns>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
