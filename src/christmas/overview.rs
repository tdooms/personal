use yew::*;
use cobul::*;
use cobul::icons::Solid;
use yew_router::prelude::*;
use crate::nav::Route;
use super::songs::Songs;

#[function_component(Christmas)]
pub fn christmas() -> Html {
    let nav = use_navigator().unwrap();

    let c = move |name: &'static str| {
        let nav = nav.clone();
        Callback::from(move |_| nav.push(&Route::Play { id: name.into() }))
    };

    let middle = html! {
        <Box>
        <Title> {"Easy"} </Title>
        <Buttons>
            <Button icon={Solid::Play} text="Christmas" color={Color::Success} fullwidth=true click={c("christmas1")}/>
            <Button icon={Solid::Play} text="Animals" color={Color::Danger} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Celebrities" color={Color::Warning} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Foods" color={Color::Link} fullwidth=true disabled=true />
        </Buttons>
        </Box>
    };

    let right = html! {
        <Box>
        <Title> {"Hard"} </Title>
        <Buttons>
            <Button icon={Solid::Play} text="Christmas" color={Color::Success} fullwidth=true click={c("christmas2")}/>
            <Button icon={Solid::Play} text="Movies" color={Color::Danger} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Monuments" color={Color::Warning} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Bands" color={Color::Link} fullwidth=true disabled=true />
        </Buttons>
        </Box>
    };

    let left = html! {
        <Box>
        <Title> {"Rules"} </Title>
        <Content>
        <ul>
            <li> {"No talking and writing during the round"} </li>
            <li> {"Each correct answer is worth 1 point"} </li>
            <li> {"The (fully) correct order is worth 2 bonus points"} </li>
        </ul>
        </Content>
        <div style="height: 38px"/>
        <Buttons>
            <Button icon={Solid::Play} text="Demo" color={Color::Light} fullwidth=true click={c("fruits")} />
        </Buttons>
        </Box>
    };

    html! {
        <Container>
        <Columns class="mt-5 mb-4">
            <Column class="has-text-centered"> <Title> {"Santa's Memory Game"} </Title> </Column>
        </Columns>

        <Columns>
        <Column> {left} </Column>
        <Column> {middle} </Column>
        <Column> {right} </Column>
        </Columns>
        <hr/>
        <Columns class="mt-5 mb-4">
            <Column class="has-text-centered"> <Title> {"Merry Shower Melody"} </Title> </Column>
        </Columns>

        <Songs />

        </Container>
    }
}