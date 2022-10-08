use std::fmt::Write;

use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize, PartialEq, Clone)]
pub struct Config {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Data,
    pub options: Options,
}

#[derive(Serialize, PartialEq, Clone)]
pub struct Options {}

#[derive(Serialize, PartialEq, Clone)]
pub struct Data {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Serialize, PartialEq, Clone)]
pub struct Dataset {
    pub label: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    #[serde(rename = "borderColor")]
    pub border_color: String,
    pub data: Vec<f32>,
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub id: String,
    pub config: Config,
}

#[function_component(Chart)]
pub fn chart_component(props: &ChartProps) -> Html {
    let config_string = serde_json::to_string(&props.config).unwrap();

    let mut script = String::new();
    write!(
        script,
        "
            const {} = new Chart(
                document.getElementById('{}'),
                {}
            );
        ",
        props.id, props.id, config_string
    )
    .unwrap();

    html!(
        <>
            <script>{ script }</script>
            <canvas id={ props.id.clone() }></canvas>
        </>
    )
}
