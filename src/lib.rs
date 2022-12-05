use yew::prelude::*;

mod easy;

pub use easy::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Config {
    Easy(ConfigEasy),
    Raw(String),
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub id: String,
    pub config: Config,
}

#[function_component(Chart)]
pub fn chart_component(props: &ChartProps) -> Html {
    let config_string = match &props.config {
        Config::Easy(config) => serde_json::to_string(config).unwrap(),
        Config::Raw(config) => config.clone(),
    };

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
