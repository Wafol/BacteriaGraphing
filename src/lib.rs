use js_sys::Math::random;
use wasm_bindgen::prelude::*;
use wasm_sockets::{self, WebSocketError};

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

extern crate web_sys;


///////////////Code for implementing log////////////////////
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
///////////////////////////////////////////////////////////

////////////needed only when you generate your data here in rust///////////////////
#[derive(Copy, Clone)]
struct OneBacDataTemplate {
    
    data: [i32; 7],
}

#[derive(Default)]
struct BacData {
    data_vec: Vec<OneBacDataTemplate>,
}

fn loadData() -> BacData {
let mut bac_data = BacData::default();

    for i in 0..=1000000 {
        let one_bac_data = OneBacDataTemplate {
            data: [i, (random()*1000.0) as i32, (random()*1000.0) as i32, (random()*1000.0) as i32, (random()*1000.0) as i32, (random()*1000.0) as i32, (random()*1000.0) as i32],
        };

        bac_data.data_vec.push(one_bac_data);
    }

    return bac_data;
}
////////////////////////////////////////////////////////////////////////

#[wasm_bindgen]
pub fn mainRust() {
    initChart();

    //Result is not handled, sorry I am lazy!!!
    let handle_this = configureWebSocketClient();
}

//This is called again in the websocket client, here is it bcs I need to fill the background white and set up the mesh so I dont have to do it again
fn initChart() {
    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root_drawing_area = canvas_backend.into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root_drawing_area)
    .set_label_area_size(LabelAreaPosition::Left, 50)
    .set_label_area_size(LabelAreaPosition::Bottom, 50)
    
    .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
    .unwrap();

    chart.configure_mesh().draw().unwrap();
}

fn configureWebSocketClient() -> Result<(), WebSocketError> {
    let mut client = wasm_sockets::EventClient::new("ws://localhost:8000/ws")?;
    client.set_on_error(Some(Box::new(|error| {
        //error!("{:#?}", error);
    })));
    client.set_on_connection(Some(Box::new(|client: &wasm_sockets::EventClient| {
        //info!("{:#?}", client.status);
        //info!("Sending message...");
        //console_log!("Sending message...");

        //client.send_string("Hello, World!").unwrap();
        //client.send_binary(vec![20]).unwrap();
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

                    ///////////////////////INIT CHART///////////////////////
                    let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
                    let root_drawing_area = canvas_backend.into_drawing_area();

                    let mut chart = ChartBuilder::on(&root_drawing_area)
                    .set_label_area_size(LabelAreaPosition::Left, 50)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    
                    .build_cartesian_2d((1..1000).log_scale(), (1..1000).log_scale())
                    .unwrap();
                    ///////////////////////////////////////////////////////

                    // According to this is the "one_bac_data_arr" array indexed.
                    // n = 0, t = 1, w = 2, fsc1_h = 3, fsc1_a = 4, fsc2_h = 5, fsc2_a = 6
                    let one_bac_data_arr = processBacBinaryData(data);
                    let point_data = [(one_bac_data_arr[1], one_bac_data_arr[2])];

                    chart.draw_series(
                        //point_data.iter().map(|point| Circle::new(*point, 5, &BLUE)),
                        point_data.iter().map(|point| Pixel::new(*point, RGBAColor(0, 0, 255, 1.0))),
                    ).unwrap();

                    //client.send_string("Send next message").unwrap();  //this is called when the server waits for response from client
                    checkTime();
                }
            }

            //console_log!("New Message: {:#?}", message);
        },
    )));

    Ok(())
}


//this fn gets data that the client recieves and converts it to the desired format
//here I am expecting to get 28xu8 values each 4bytes are one bacteria property (there are 7 of these) !!if OTHER fromat is inserted the fn will NOT WORK
fn processBacBinaryData(data: Vec<u8>) -> [i32; 7] {
    let mut one_bac_data_arr: [i32; 7] = [0; 7];

    for i in 0..(data.len()/4) {
        let mut num32 = 0;

        for j in 0..4 {
            num32 <<= 8;
            num32 |= data[(i*4) + j] as i32;
        }
        
        one_bac_data_arr[i] = num32;
    }

    return one_bac_data_arr;
}

//static vars used by checkTime();
static mut COUNTER: usize = 0;
static mut THEN: f64 = 0.0;

//this function is for calculating the speed of the bacteria drawing
//call everytime new bacteria is drawn
fn checkTime() {
    unsafe {
        if COUNTER == 0 {
            THEN = js_sys::Date::now();
        }
        
        COUNTER += 1;

        let per_bac = 10000;

        if COUNTER >= per_bac {
            console_log!("SPEED: {} cells per {} ms", per_bac, js_sys::Date::now() - THEN);
            COUNTER = 0;
        }
    }
}