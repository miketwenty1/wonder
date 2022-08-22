// use actix_rt::System;
// use awc::Client;
use serde_json::Value;
//use std::convert::TryInto;
use wasm_bindgen::prelude::*;

// fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
//     v.try_into()
//         .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn pg() {
    // System::new().block_on(async {
    //     let client = Client::default();

    //     let res = client
    //         .get("http://localhost:8080/hey")    // <- Create request builder
    //         .insert_header(("User-Agent", "Actix-web"))
    //         .send()                             // <- Send http request
    //         .await;

    //     println!("Response: {:?}", res);        // <- server http response
    // });

    //println!()
    let request = ehttp::Request::get("http://localhost:8081/hello/tidwell");
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        match result {
            Ok(res) => {
                let bytes = res.bytes;
                let v: Value = serde_json::from_slice(&bytes).unwrap();
                println!("Status code: {:?}", &v);
                console_log!(
                    "Hello you found an item {}!",
                    &v["character"]["items"][1].as_str().unwrap()
                );
            }
            Err(e) => {
                //println!("Status code: {:?}", v);
                //console::log_1(&"Hello using web-sys".into());
                console_log!(
                    "Cant connect to server bro! please report this error: {}",
                    e
                );
            }
        }

        //let bytes = demo(result.unwrap().bytes);
    });
}
