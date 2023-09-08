use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// Type alias for the result of a drawing function.

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;


#[wasm_bindgen]
pub fn draw() {
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area = canvas_backend.into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        
        .build_cartesian_2d((1..100).log_scale(), (1..100).log_scale())
        .unwrap();
    
    chart.configure_mesh().draw().unwrap();

    chart.draw_series(
        DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)),
    ).unwrap();

}

const DATA1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];

