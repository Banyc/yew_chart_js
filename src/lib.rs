use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;

mod chart_js;
mod easy;

pub use chart_js::*;
pub use easy::*;

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub id: String,
    pub config: JsValue,
}

#[function_component(Chart)]
pub fn chart_component(props: &ChartProps) -> Html {
    let chart_js = use_state(|| None);

    use_effect_with_deps(
        {
            let chart_js = chart_js.clone();
            let id = props.id.clone();
            let config = props.config.clone();
            move |_| {
                let canvas = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&id)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();

                chart_js.set(Some(chart_js::Chart::new(&canvas, config)));
            }
        },
        (),
    );

    html!(
        <>
            <canvas id={ props.id.clone() }></canvas>
        </>
    )
}
