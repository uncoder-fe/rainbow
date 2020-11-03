extern crate wasm_bindgen;

use std::{f64, i32, u32};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const colors: [[i32; 3]; 8] = [
    [255, 0, 0],
    [255, 255, 0],
    [0, 255, 0],
    [0, 255, 255],
    [0, 0, 255],
    [255, 0, 255],
    [255, 0, 0],
    [255, 255, 0],
];

const colors2: [[i32; 3]; 8] = [
    [255, 255, 0],
    [255, 0, 0],
    [255, 0, 255],
    [0, 0, 255],
    [0, 255, 255],
    [0, 255, 0],
    [255, 255, 0],
    [255, 0, 0],
];

fn arrayToString(array: &[i32]) -> String {
    let stuff_str: Vec<String> = array.iter().map(|n| n.to_string()).collect();
    return stuff_str.join(",");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    const width: u32 = 350;
    const height: u32 = 350;
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let p_dom = document.create_element("p")?;
    // p_dom.set_inner_html("<del>Hello from Rust!</del>");
    // body.append_child(&p_dom)?;
    // Manufacture the element we're gonna append
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(width);
    canvas.set_height(height);
    canvas.style().set_property("border", "1px solid red")?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    ctx.begin_path();
    ctx.set_font("50px any");
    // 画布渐变
    let gradient = ctx.create_linear_gradient(0.0, 0.0, 350.0, 0.0);
    for i in 0..7 as usize {
        gradient
            .add_color_stop(
                (i as f32) / 7.0,
                &format!("rgb({})", arrayToString(&colors[7 - i])),
            )
            .expect("完犊子，设置颜色失败了");
    }
    ctx.set_fill_style(&gradient);
    ctx.fill_text("薇薇安，好酷。", 0.0, 200.0)
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
#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: i32,
    height: i32,
    p_data: &[i8],
) -> Result<(), JsValue> {
    log("canvas像素", &p_data.len().to_string());
    let l = p_data.len();
    let mut data: Vec<u8> = vec![0; l];
    // 染色范围，切7份
    for i in 0..350 as usize {
        for j in 0..7 as usize {
            // log("j", &j.to_string());
            let red_is_equal: bool = colors2[(j + 1)][0] == colors2[j][0];
            let red = (if !red_is_equal {
                colors2[(j + 1)][0] - colors2[j][0]
            } else {
                colors2[j][0]
            }) as f64;
            let green_is_equal = colors2[(j + 1)][1] == colors2[j][1];
            let green = (if !green_is_equal {
                colors2[(j + 1)][1] - colors2[j][1]
            } else {
                colors2[j][1]
            }) as f64;
            let blue_is_equal = colors2[(j + 1)][2] == colors2[j][2];
            let blue = (if !blue_is_equal {
                colors2[(j + 1)][2] - colors2[j][2]
            } else {
                colors2[j][2]
            }) as f64;
            let range = get_width_range(if j == 0 { 0 } else { 50 * j }, 50, 350, i);
            for z in (range.0)..=(range.1) {
                let p = (z as f64 - range.0 as f64) / 50 as f64;
                if p_data[(z * 4 + 3)] != 0 {
                    data[(z * 4)] = (if red_is_equal {
                        red
                    } else {
                        if red >= 0.0 {
                            red * p
                        } else {
                            255.0 + red * p
                        }
                    }) as u8;
                    data[(z * 4 + 1)] = (if green_is_equal {
                        green
                    } else {
                        if green >= 0.0 {
                            green * p
                        } else {
                            255.0 + green * p
                        }
                    }) as u8;
                    data[(z * 4 + 2)] = (if blue_is_equal {
                        blue
                    } else {
                        if blue >= 0.0 {
                            blue * p
                        } else {
                            255.0 + blue * p
                        }
                    }) as u8;
                    data[(z * 4 + 3)] = 255 as u8;
                }
            }
        }
    }
    // The real workhorse of this algorithm, generating pixel data
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut data),
        width as u32,
        height as u32,
    )?;
    ctx.put_image_data(&data, 0.0, 0.0);
    
    Ok(())
}

fn get_width_range(
    start: usize,
    width: usize,
    canvas_width: usize,
    line_number: usize,
) -> (usize, usize) {
    let start_index = start + line_number * canvas_width;
    let end_index = start_index + width;
    return (start_index, end_index - 1);
}
