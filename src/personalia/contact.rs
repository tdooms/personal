use cobul::*;
use cobul::icons::{Brands, Solid};

use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ContactData {
    email: AttrValue,
    phone: AttrValue,
    linkedin: AttrValue,
    github: AttrValue,
}

#[function_component(Contact)]
pub fn contact(props: &ContactData) -> Html {
    let ContactData {email, phone, linkedin, github} = props.clone();

    html! {
        <div class="has-text-centered">
        <Block>
            <Icon icon={Solid::Envelope} color={TextColor::Danger} />
            <span> {" "}{email.clone()} </span>
        </Block>
        <Block>
            <Icon icon={Solid::Phone} color={TextColor::Danger} />
            <span> {" "}{phone.clone()} </span>
        </Block>
        <Block>
            <Icon icon={Brands::Linkedin} color={TextColor::Danger} />
            <span> <a href={linkedin.clone()}> {" "}{"LinkedIn"} </a> </span>
        </Block>
        <Block>
            <Icon icon={Brands::Github} color={TextColor::Danger} />
            <span> <a href={github.clone()}> {" "}{"GitHub"} </a> </span>
        </Block>
        </div>
    }
}
