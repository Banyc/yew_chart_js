use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let labels = vec!["January", "February", "March", "April", "May", "June"];
    let labels = labels
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let data = yew_chart_js::Data {
        labels: Some(labels),
        datasets: vec![yew_chart_js::Dataset {
            label: "My First dataset".to_string(),
            background_color: "rgb(255, 99, 132)".to_string(),
            border_color: "rgb(255, 99, 132)".to_string(),
            data: yew_chart_js::DatasetData::Scalars(vec![
                yew_chart_js::Number::Int(0),
                yew_chart_js::Number::Int(10),
                yew_chart_js::Number::Int(5),
                yew_chart_js::Number::Int(2),
                yew_chart_js::Number::Int(10),
                yew_chart_js::Number::Int(30),
                yew_chart_js::Number::Int(45),
            ]),
        }],
    };
    let config = yew_chart_js::Config::Easy(yew_chart_js::ConfigEasy {
        type_: "line".to_string(),
        data,
        options: yew_chart_js::Options { scales: None },
    });

    html! {
        <div>
            <yew_chart_js::Chart id="myChart" config={ config } />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
