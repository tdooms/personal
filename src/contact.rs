use cobul::*;
use cobul::fa::{Brands, Solid};
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ContactData {
    email: &'static str,
    phone: &'static str,
    address: &'static str,
    linkedin: &'static str,
    github: &'static str,
}

#[function_component(Contact)]
pub fn contact(props: &ContactData) -> Html {
    html! {
        <div class="has-text-centered">
        <Block>
            <Icon icon={Solid::Envelope} color={TextColor::Danger} />
            <span> {" "}{props.email} </span>
        </Block>
        <Block>
            <Icon icon={Solid::Phone} color={TextColor::Danger} />
            <span> {" "}{props.phone} </span>
        </Block>
        <Block>
            <Icon icon={Solid::MapLocation} color={TextColor::Danger} />
            <span> {" "}{props.address} </span>
        </Block>
        <Block>
            <Icon icon={Brands::Linkedin} color={TextColor::Danger} />
            <span> <a href={props.linkedin}> {" "}{"LinkedIn"} </a> </span>
        </Block>
        <Block>
            <Icon icon={Brands::Github} color={TextColor::Danger} />
            <span> <a href={props.github}> {" "}{"GitHub"} </a> </span>
        </Block>

        </div>
    }
}
