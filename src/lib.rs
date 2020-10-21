extern crate wasm_bindgen;

use std::{f64, i32, u32};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    const width: u32 = 350;
    const height: u32 = 350;
    let colors = (
        ["255", "0", "0"],
        ["255", "255", "0"],
        ["0", "255", "0"],
        ["0", "255", "255"],
        ["0", "0", "255"],
        ["255", "0", "255"],
        ["255", "0", "0"],
        ["255", "255", "0"],
    );
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let p_dom = document.create_element("p")?;
    p_dom.set_inner_html("<del>Hello from Rust!</del>");
    body.append_child(&p_dom)?;
    // Manufacture the element we're gonna append
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(width);
    canvas.set_height(height);
    canvas.style().set_property("border", "1px solid red")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    context.begin_path();
    context.set_font("50px any");
    // 画布渐变
    let gradient = context.create_linear_gradient(0.0, 0.0, 350.0, 0.0);
    gradient.add_color_stop(0.0, &format!("rgb({})", (colors.7).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(1.0 / 7.0, &format!("rgb({})", (colors.6).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(2.0 / 7.0, &format!("rgb({})", (colors.5).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(3.0 / 7.0, &format!("rgb({})", (colors.4).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(4.0 / 7.0, &format!("rgb({})", (colors.3).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(5.0 / 7.0, &format!("rgb({})", (colors.2).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(6.0 / 7.0, &format!("rgb({})", (colors.1).join(","))).expect("完犊子，设置颜色失败了");
    gradient.add_color_stop(7.0 / 7.0, &format!("rgb({})", (colors.0).join(","))).expect("完犊子，设置颜色失败了");
    context.set_fill_style(&gradient);
    context
        .fill_text("薇薇安，好酷。", 0.0, 200.0)
        .expect("完犊子，绘制失败了");
    body.append_child(&canvas)?;
    // 打印到控制台里
    log("webassembly says:", "使用rust绘制成功");

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(l: &str, s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
