mod contact;
mod profile;

use yew::*;
use contact::*;
use profile::*;


#[derive(serde::Deserialize, Properties, PartialEq, Clone, Debug)]
pub struct PersonaliaData {
    pub profile: ProfileData,
    pub contact: ContactData,
}

#[function_component(Personalia)]
pub fn personalia() -> Html {
    let str = include_str!("data.json");
    let PersonaliaData{profile, contact} = serde_json::from_str(str).unwrap();

    html! { <> <Profile ..profile/> <Contact ..contact/> </> }
}