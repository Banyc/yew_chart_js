use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Config {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Data,
    pub options: Options,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Options {
    pub scales: Option<Scales>,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Scales {
    pub x: Axis,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Axis {
    #[serde(rename = "type")]
    pub type_: String,
    pub position: Option<String>,
    pub time: Option<Time>,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Time {
    pub unit: Option<String>,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Data {
    pub labels: Option<Vec<String>>,
    pub datasets: Vec<Dataset>,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Dataset {
    pub label: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    #[serde(rename = "borderColor")]
    pub border_color: String,
    pub data: DatasetData,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum DatasetData {
    Scalars(Vec<Number>),
    Points(Vec<Point>),
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Point {
    pub x: Number,
    pub y: Number,
}

#[derive(Serialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Number {
    Float(f64),
    Int(i64),
    UInt(u64),
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub id: String,
    pub config: Config,
}

#[function_component(Chart)]
pub fn chart_component(props: &ChartProps) -> Html {
    let config_string = serde_json::to_string(&props.config).unwrap();

    let script = format!(
        "
            var {} = new Chart(
                document.getElementById('{}'),
                {}
            );
        ",
        props.id, props.id, config_string
    );

    html!(
        <>
            <script>{ script }</script>
            <canvas id={ props.id.clone() }></canvas>
        </>
    )
}
