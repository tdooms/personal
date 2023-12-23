use yew::*;
use cobul::*;
use cobul::icons::Solid;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub done: Callback<()>,
}

#[function_component(Wait)]
pub fn wait(props: &Props) -> Html {
    let done = props.done.clone();

    html! {
        <Container>
        <Columns centered=true vcentered=true>
        <Column class="has-text-centered" size={ColumnSize::Is4}>
            <Block class="pt-6" />
            <Block class="pb-6" />
            <b style="font-weight: 800" class="title is-3"> {"Write Down Your Answers"} </b>
            <Buttons class="mt-6">
                <Button click={done} light=true color={Color::Danger} text="Summary" icon={Solid::Eye} fullwidth=true />
            </Buttons>
        </Column>
        </Columns>
        </Container>
    }
}