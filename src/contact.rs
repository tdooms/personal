use cobul::*;
use cobul::icons::{Brands, Solid};
use implicit_clone::unsync::IString;
use yew::*;

#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct ContactData {
    email: IString,
    phone: IString,
    address: IString,
    linkedin: IString,
    github: IString,
}

#[function_component(Contact)]
pub fn contact(props: &ContactData) -> Html {
    html! {
        <div class="has-text-centered">
        <Block>
            <Icon icon={Solid::Envelope} color={TextColor::Danger} />
            <span> {" "}{props.email.clone()} </span>
        </Block>
        <Block>
            <Icon icon={Solid::Phone} color={TextColor::Danger} />
            <span> {" "}{props.phone.clone()} </span>
        </Block>
        <Block>
            <Icon icon={Solid::MapLocation} color={TextColor::Danger} />
            <span> {" "}{props.address.clone()} </span>
        </Block>
        <Block>
            <Icon icon={Brands::Linkedin} color={TextColor::Danger} />
            <span> <a href={props.linkedin.clone()}> {" "}{"LinkedIn"} </a> </span>
        </Block>
        <Block>
            <Icon icon={Brands::Github} color={TextColor::Danger} />
            <span> <a href={props.github.clone()}> {" "}{"GitHub"} </a> </span>
        </Block>

        </div>
    }
}
