use yew::*;
use cobul::*;
use cobul::icons::Solid;


#[derive(Clone, Debug, PartialEq, Properties)]
struct Props {
    pub src: AttrValue,
    pub title: AttrValue,
    pub answer: AttrValue
}

#[function_component(Song)]
fn song(props: &Props) -> Html {
    let Props {src, title, answer} = props.clone();
    let model = use_model(|| false);
    let text = if *model {answer} else {"Reveal".into()};
    let icon = if *model {Solid::EyeSlash} else {Solid::Eye};

    html! {
        <Column size={ColumnSize::Is4}>
        <Box class="is-flex is-flex-direction-column is-align-content-center">
            <Columns>
            <Column> <Title> {title} </Title> </Column>
            <Column size={ColumnSize::Is8}> <Button {model} {text} {icon} color={Color::White} fullwidth=true /> </Column>
            </Columns>
            <audio controls=true>
                <source {src} type="audio/mpeg" />
            </audio>
        </Box>
        </Column>
    }
}

#[function_component(Songs)]
pub fn songs() -> Html {
    html! {
        <Columns multiline=true>
            <Column>
            <Box>
            <Title> {"Rules"} </Title>
            <div style="height:20px" />
            <Content> <p> {"Guess the Christmas song, 3 points per answer"} </p> </Content>
            </Box>
            </Column>

            <Song src="/static/christmas/1.mp3" title="Demo" answer="Jingle Bell Rock" />
            <Song src="/static/christmas/2.mp3" title="Song 1" answer="Feliz Navidad" />
            <Song src="/static/christmas/3.mp3" title="Song 2" answer="All I Want For Christmas" />
            <Song src="/static/christmas/4.mp3" title="Song 3" answer="Last Christmas" />
            <Song src="/static/christmas/5.mp3" title="Song 4" answer="Wonderful Christmastime" />
        </Columns>
    }
}