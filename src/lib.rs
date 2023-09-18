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


pub struct StructWithRef<'a> {
    ref_str: &'a String,
}

impl<'a> StructWithRef<'a> {
    pub fn on(root: &'a String) -> Self {
        Self {
            ref_str: root,
        }
    }
}

// pub struct StructWithRef {
//     ref_str: String,
// }

// impl StructWithRef {
//     pub fn on(root: String) -> Self {
//         Self {
//             ref_str: root,
//         }
//     }
// }


//static root_drawing_area2: DrawingArea2<CanvasBackend, Shift>;

pub fn draw2() -> Result<(), WebSocketError> {
    ///////////////////////INIT CHART///////////////////////////////////////////////////////////
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area: DrawingArea<CanvasBackend, Shift> = canvas_backend.into_drawing_area();

    let mut chart: ChartContext<'static, CanvasBackend, _> = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        
        .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
        .unwrap();    
    
    
    let bac_data = loadData();
    let data_vec = bac_data.data_vec;

    let then = js_sys::Date::now();
    
    chart.draw_series(
        //data_vec.iter().map(|one_bac: &OneBacDataTemplate| Circle::new((one_bac.w, one_bac.t), 1, &BLUE)),
        data_vec.iter().map(|one_bac: &OneBacDataTemplate|{

            Pixel::new((one_bac.data[1], one_bac.data[2]), RGBAColor(0, 0, 255, 0.5))
        }),
        //DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)),
    ).unwrap();
    let now = js_sys::Date::now();
    //now - then
    

    Ok(())

    //root_drawing_area.present().unwrap();

}


fn create_and_configure_client(mut chart: ChartContext<'static, CanvasBackend, Shift>) -> Result<(), WebSocketError> {
    //let x = Box::new(42);

    let mut rc_chart = Rc::new(chart);
    let mut cloned_chart = rc_chart.clone();


    let mut client = wasm_sockets::EventClient::new("ws://localhost:8000/ws")?;
    

    client.set_on_error(Some(Box::new(move |error| {
            //error!("{:#?}", error);
            cloned_chart.plotting_area();
            //chart.plotting_area();    
    })));

    client.set_on_connection(Some(Box::new(|client: &wasm_sockets::EventClient| {
        //info!("{:#?}", client.status);
        //info!("Sending message...");
        //console_log!("Sending message...");

        client.send_string("Hello, World!").unwrap();
        client.send_binary(vec![20]).unwrap();
    })));
    client.set_on_close(Some(Box::new(|_evt| {
        //info!("Connection closed");
        //console_log!("Connection closed");
    })));
    client.set_on_message(Some(Box::new(
        |client: &wasm_sockets::EventClient, message: wasm_sockets::Message| {
            //info!("New Message: {:#?}", message);

            match message {
                wasm_sockets::Message::Text(text) => {
                    // Handle text message (e.g., display or process the text)
                    //console_log!("Received text message: {}", text);
                }
                wasm_sockets::Message::Binary(data) => {
                    // Handle binary message (e.g., process binary data)
                    // Note: 'data' is a Vec<u8> containing binary data.
                    //console_log!("Received binary message with {} bytes", data.len());
                    //console_log!("Received binary message {} {} {} {}", data[0], data[1], data[2], data[3]); 

                    let mut arr: [i32; 7] = [0; 7];
                    for i in 0..(data.len()/4) {
                        let mut num32 = 0;

                        for j in 0..4 {
                            num32 <<= 8;
                            num32 |= data[(i*4) + j] as i32; //(i*4) + j
                        }
                        
                        arr[i] = num32;
                    }

                    let draw_data: [(i32, i32); 1] = [(arr[1], arr[2])];
                    
                    //let b = *a;
/*
                    let mut chart_ref = &mut *chart;
                    chart_ref.draw_series(
                        draw_data.iter().map(|point| Circle::new(*point, 5, &BLUE)),
                    ).unwrap();
                    */

                    //console_log!("one cell data: {:?}", arr);
                }
            }

            //console_log!("New Message: {:#?}", message);
        },
    )));


    Ok(())
}








pub fn draw3() -> Result<(), WebSocketError> {
    ///////////////////////INIT CHART///////////////////////////////////////////////////////////
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area = canvas_backend.into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        
        .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
        .unwrap();
    
    chart.configure_mesh()
    .x_desc("x = Array::range(1., 7., 0.1);")
    .y_desc("y = f(x)")
    .draw().unwrap();

    ///////////////////////////////////////////////////////////////////////////////////////////

    
/*
    let mut client = wasm_sockets::EventClient::new("ws://localhost:8000/ws")?;
    client.set_on_error(Some(Box::new(|error| {
        //error!("{:#?}", error);
    })));
    client.set_on_connection(Some(Box::new(|client: &wasm_sockets::EventClient| {
        //info!("{:#?}", client.status);
        //info!("Sending message...");
        //console_log!("Sending message...");

        client.send_string("Hello, World!").unwrap();
        client.send_binary(vec![20]).unwrap();
    })));
    client.set_on_close(Some(Box::new(|_evt| {
        //info!("Connection closed");
        //console_log!("Connection closed");
    })));
    client.set_on_message(Some(Box::new(
        |client: &wasm_sockets::EventClient, message: wasm_sockets::Message| {
            //info!("New Message: {:#?}", message);

            match message {
                wasm_sockets::Message::Text(text) => {
                    // Handle text message (e.g., display or process the text)
                    //console_log!("Received text message: {}", text);
                }
                wasm_sockets::Message::Binary(data) => {
                    // Handle binary message (e.g., process binary data)
                    // Note: 'data' is a Vec<u8> containing binary data.
                    //console_log!("Received binary message with {} bytes", data.len());
                    //console_log!("Received binary message {} {} {} {}", data[0], data[1], data[2], data[3]); 

                    let mut arr: [i32; 7] = [0; 7];
                    for i in 0..(data.len()/4) {
                        let mut num32 = 0;

                        for j in 0..4 {
                            num32 <<= 8;
                            num32 |= data[(i*4) + j] as i32; //(i*4) + j
                        }
                        
                        arr[i] = num32;
                    }

                    let draw_data: [(i32, i32); 1] = [(arr[1], arr[2])];
                    let chart = *boxed_chart;
                    chart.draw_series(
                        //data_vec.iter().map(|one_bac: &OneBacDataTemplate| Circle::new((one_bac.w, one_bac.t), 1, &BLUE)),
                        
                        /*data_vec.iter().map(|one_bac: &OneBacDataTemplate|{
                
                            Pixel::new((one_bac.data[1], one_bac.data[2]), RGBAColor(0, 0, 255, 0.5))
                        }),*/
                        draw_data.iter().map(|point| Circle::new(*point, 5, &BLUE)),
                    ).unwrap();
                    
                    //console_log!("one cell data: {:?}", arr);
                }
            }

            //console_log!("New Message: {:#?}", message);
        },
    )));
*/

/*    
    let bac_data = loadData();
    let data_vec = bac_data.data_vec;

    let then = js_sys::Date::now();
    chart.draw_series(
        //data_vec.iter().map(|one_bac: &OneBacDataTemplate| Circle::new((one_bac.w, one_bac.t), 1, &BLUE)),
        data_vec.iter().map(|one_bac: &OneBacDataTemplate|{

            Pixel::new((one_bac.data[1], one_bac.data[2]), RGBAColor(0, 0, 255, 0.5))
        }),
        //DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)),
    ).unwrap();
    let now = js_sys::Date::now();
    //now - then
*/

    Ok(())
    //root_drawing_area.present().unwrap();

}

const DATA1: [(i32, i32); 30] =  [(3, 1), (2, 3), (4, 2), (3, 0), (6, 5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
