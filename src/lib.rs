
use js_sys::{Math::random, Date};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// Type alias for the result of a drawing function.

use plotters::{prelude::*, data};
use plotters_canvas::CanvasBackend;

extern crate web_sys;
use web_sys::console;
use std::time::{Duration, SystemTime, UNIX_EPOCH};


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


struct OneBacDataTemplate {
    n: i32,
    t: i32,
    w: i32,
    fsc1_h: i32,
    fsc1_a: i32,
    fsc2_h: i32,
    fsc2_a: i32,
}

#[derive(Default)]
struct BacData {
    data_vec: Vec<OneBacDataTemplate>,
}


fn loadData() -> BacData{
let mut bac_data = BacData::default();

    for i in 0..=1000000 {
        let one_bac_data = OneBacDataTemplate {
            n: i,
            t: (random()*10000.0) as i32,
            w: (random()*10000.0) as i32,
            fsc1_h: (random()*10000.0) as i32,
            fsc1_a: (random()*10000.0) as i32,
            fsc2_h: (random()*10000.0) as i32,
            fsc2_a: (random()*10000.0) as i32,
        };

        bac_data.data_vec.push(one_bac_data);
    }

    return bac_data;
}


#[wasm_bindgen]
pub fn draw() -> f64 {
    ///////////////////////INIT CHART///////////////////////////////////////////////////////////
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area: DrawingArea<CanvasBackend, plotters::coord::Shift> = canvas_backend.into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart: ChartContext<'_, CanvasBackend, Cartesian2d<LogCoord<i32>, LogCoord<i32>>> = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        
        .build_cartesian_2d((1..10000).log_scale(), (1..10000).log_scale())
        .unwrap();
    
    chart.configure_mesh().draw().unwrap();
    ///////////////////////////////////////////////////////////////////////////////////////////


    let bac_data = loadData();
    let data_vec = bac_data.data_vec;

    let then = js_sys::Date::now();

    chart.draw_series(
        data_vec.iter().map(|one_bac: &OneBacDataTemplate| Circle::new((one_bac.w, one_bac.t), 1, &BLUE)),
        //DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)),
    ).unwrap();

    let now = js_sys::Date::now();
    now - then

    //root_drawing_area.present().unwrap();

}


const DATA1: [(i32, i32); 30] =  [(3, 1), (2, 3), (4, 2), (3, 0), (6, 5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];