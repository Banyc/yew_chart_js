use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Chart;

    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &web_sys::HtmlCanvasElement, config: JsValue) -> Chart;
}
