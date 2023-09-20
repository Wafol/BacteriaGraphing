use js_sys::{Math::random, Date};
use plotters::coord::Shift;
use wasm_bindgen::{prelude::*, closure};
use web_sys::HtmlCanvasElement;
use wasm_sockets::{self, WebSocketError};

/// Type alias for the result of a drawing function.

use plotters::{prelude::*, data, chart};
use plotters_canvas::CanvasBackend;

extern crate web_sys;
use web_sys::console;
use std::{default, error};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

#[macro_use]
extern crate lazy_static;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

enum DataNames {
    n = 0,
    t = 1,
    w = 2,
    fsc1_h = 3,
    fsc1_a = 4,
    fsc2_h = 5,
    fsc2_a = 6
}

struct OneBacDataTemplate {
    data: [i32; 7],
}

#[derive(Default)]
struct BacData {
    data_vec: Vec<OneBacDataTemplate>,
}


fn loadData() -> BacData{
let mut bac_data = BacData::default();

    for i in 0..=100000 {
        let one_bac_data = OneBacDataTemplate {
            data: [i, (random()*10000.0) as i32, (random()*10000.0) as i32, (random()*10000.0) as i32, (random()*10000.0) as i32, (random()*10000.0) as i32, (random()*10000.0) as i32],
        };

        bac_data.data_vec.push(one_bac_data);
    }

    return bac_data;
}

#[wasm_bindgen]
pub fn draw() {
    draw2();
}


static mut COUNTER: usize = 0;

pub fn draw2() -> Result<(), WebSocketError> {
    ///////////////////////INIT CHART///////////////////////////////////////////////////////////
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area = canvas_backend.into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root_drawing_area)
    .set_label_area_size(LabelAreaPosition::Left, 50)
    .set_label_area_size(LabelAreaPosition::Bottom, 50)
    
    .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
    .unwrap();

    chart.configure_mesh().draw().unwrap();

    
    let mut client = wasm_sockets::EventClient::new("ws://localhost:8000/ws")?;

    client.set_on_message(Some(Box::new(
        move |client: &wasm_sockets::EventClient, message: wasm_sockets::Message| {

            let canvas_backend2 = CanvasBackend::new("canvas").expect("cannot find canvas");
            let root_drawing_area2 = canvas_backend2.into_drawing_area();

            let mut chart = ChartBuilder::on(&root_drawing_area2)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            
            .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
            .unwrap();

            unsafe {
            let data2 = [DATA1[COUNTER]];

            chart.draw_series(
                //data_vec.iter().map(|one_bac: &OneBacDataTemplate| Circle::new((one_bac.w, one_bac.t), 1, &BLUE)),
                /* 
                data_vec.iter().map(|one_bac: &OneBacDataTemplate| {
                    //Pixel::new((one_bac.data[1], one_bac.data[2]), RGBAColor(0, 0, 255, 1.0))
                    Circle::new((one_bac.data[1], one_bac.data[2]), 5, &BLUE)
                }),*/

                data2.iter().map(|point| Circle::new(*point, 5, &BLUE)),
                //DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)),
            ).unwrap();
            
           
                COUNTER += 1;
            }
            
        },
    )));

    /* 
    let bac_data = loadData();
    let data_vec = bac_data.data_vec;

    let then = js_sys::Date::now();

    for i in 0..10 {
        let canvas_backend2 = CanvasBackend::new("canvas").expect("cannot find canvas");
        let root_drawing_area2 = canvas_backend2.into_drawing_area();

        let mut chart = ChartBuilder::on(&root_drawing_area2)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        
        .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
        .unwrap();

        let data2 = [DATA1[i]];

        chart.draw_series(
            //data_vec.iter().map(|one_bac: &OneBacDataTemplate| Circle::new((one_bac.w, one_bac.t), 1, &BLUE)),
            /* 
            data_vec.iter().map(|one_bac: &OneBacDataTemplate| {
                //Pixel::new((one_bac.data[1], one_bac.data[2]), RGBAColor(0, 0, 255, 1.0))
                Circle::new((one_bac.data[1], one_bac.data[2]), 5, &BLUE)
            }),*/

            data2.iter().map(|point| Circle::new(*point, 5, &BLUE)),
            //DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)),
        ).unwrap();


    }
    
    
    let now = js_sys::Date::now();
    //now - then
    */

    Ok(())

    //root_drawing_area.present().unwrap();

}

const DATA1: [(i32, i32); 30] =  [(3, 1), (2, 3), (4, 2), (3, 0), (6, 5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
