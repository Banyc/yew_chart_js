# `yew_chart_js`

Chart.js bindings for Yew.

## How to use

1. Include this crate in your `Cargo.toml`:

   ```toml
   [dependencies]
   yew_chart_js = { git = "https://github.com/Banyc/yew_chart_js" }
   ```

2. Put this line in the body of `<head>` tag in the `index.html` file:

   ```html
   <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
   ```

   Example:

   ```html
   <!DOCTYPE html>
   <html lang="en">
     <head>
       <title>Title</title>

       <meta charset="utf-8" />
       <meta name="viewport" content="width=device-width, initial-scale=1" />

       <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
     </head>

     <body></body>
   </html>
   ```

3. Use the `Chart` component in the `html!` macro:

   ```rust
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
       let config = yew_chart_js::Config {
           type_: "line".to_string(),
           data,
           options: yew_chart_js::Options { scales: None },
       };
   
       html! {
           <div>
               <h1>{ "Chart" }</h1>
               <yew_chart_js::Chart
                   id="myChart"
                   config={ config.to_js_value() }
               />
           </div>
       }
   }
   ```

## References

- [Chart.js](https://www.chartjs.org/)
