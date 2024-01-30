use yew::*;
use cobul::*;
use cobul::icons::Solid;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub images: Vec<(AttrValue, AttrValue)>,
    pub done: Callback<()>,
}

#[function_component(Summary)]
pub fn summary(props: &Props) -> Html {
    let done = props.done.clone();

    html! {
        <Container>
        <section class="hero is-medium has-text-centered pb-6">
        <div class="hero-body pb-6">
            <Title style="font-size:60px" size={HeaderSize::Is1}> {"Summary"} </Title>
        </div>
        </section>
        <Columns multiline=true centered=true>
        {for props.images.iter().cloned().map(|(name, src)| html! {
            <Column size={ColumnSize::Is3}>
                <Image {src} size={ImageSize::Is16by9} style="object-fit:cover;border-radius:20px"/>
                <HCenter> <p style="font-size:20px"> {name} </p> </HCenter>
            </Column>
        })}
        </Columns>

        <Buttons align={Align::Centered}>
            <Button text="Leave" icon={Solid::RightFromBracket} light=true click={done} size={Size::Medium} />
        </Buttons>

        </Container>
    }
}