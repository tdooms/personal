use yew::*;
use plotly::{Plot, Scatter, HeatMap, Layout, common::Title};
use yew::suspense::use_future;

async fn create_plot() {
    let mut plot = Plot::new();

    let trace = HeatMap::new(
        vec!["a", "b"],
        vec![2.0, 3.0],
        vec![vec![0.25, 0.75], vec![0.0, 0.5]]
    );

    plot.add_trace(trace);

    let layout = Layout::new().title(Title::new("Displaying a Chart in Yew"));
    plot.set_layout(layout);

    plotly::bindings::new_plot("plot-div", &plot).await;
}


#[function_component(Inner)]
pub fn inner() -> HtmlResult {
    let plot = use_future(create_plot)?;
    Ok(html! { })
}


#[function_component(Test)]
pub fn test() -> Html {
    let fallback = html! {"fb"};

    html! { 
        <>
        <div id="plot-div"></div>
        <Suspense {fallback}> <Inner /> </Suspense> 
        </>
    }
}