
use actix_rt::System;
use awc::Client;

pub fn pg() {
    System::new().block_on(async {
        let client = Client::default();

        let res = client
            .get("http://localhost:8080/hey")    // <- Create request builder
            .insert_header(("User-Agent", "Actix-web"))
            .send()                             // <- Send http request
            .await;

        println!("Response: {:?}", res);        // <- server http response
    });
}