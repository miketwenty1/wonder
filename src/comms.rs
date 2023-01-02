use async_channel::{Receiver, Sender};
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    game_scene::Blockheight,
    qr_code_overlay_scene::CurrentQrString,
    sharedstructs::{BlockData, InvoiceData},
    ActixServerURI, AppQr,
};

#[derive(Resource, Clone)]
pub struct HeightChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct InvoicePayChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct InvoiceCheckChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct PlayerMoveChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Default, Debug)]
pub struct Api100Receiver {
    pub c: i32,
    pub poll_trigger: i32,
}

#[derive(Resource, Default, Debug)]
pub struct Api5000Sender {
    pub c: i32,
    pub poll_trigger: i32,
}

#[allow(clippy::redundant_clone)]
pub fn setup_comm(mut commands: Commands) {
    let (tx_height, rx_height) = async_channel::bounded(1);
    let (tx_pay, rx_pay) = async_channel::bounded(1);
    let (tx_pay_check, rx_pay_check) = async_channel::bounded(1);
    let (tx_player_move, rx_player_move) = async_channel::bounded(1);

    commands.insert_resource(HeightChannel {
        tx: tx_height,
        rx: rx_height,
    });
    commands.insert_resource(InvoicePayChannel {
        tx: tx_pay,
        rx: rx_pay,
    });
    commands.insert_resource(InvoiceCheckChannel {
        tx: tx_pay_check,
        rx: rx_pay_check,
    });
    commands.insert_resource(PlayerMoveChannel {
        tx: tx_player_move,
        rx: rx_player_move,
    });

    let sender_counter = Api100Receiver {
        c: 0,
        poll_trigger: 100,
    };
    let receiver_counter = Api5000Sender {
        c: 0,
        poll_trigger: 5000,
    };

    commands.insert_resource(sender_counter);
    commands.insert_resource(receiver_counter);
}

#[allow(clippy::too_many_arguments)]
pub fn api_receiver(
    //mut commands: Commands,
    height_channel: ResMut<HeightChannel>,
    invoice_channel: ResMut<InvoicePayChannel>,
    // invoice_check_channel: ResMut<InvoiceCheckChannel>,
    // player_move_channel: ResMut<PlayerMoveChannel>,
    mut polling_counter: ResMut<Api100Receiver>,
    mut block_height_query: Query<&mut Text, With<Blockheight>>,
    mut qr_state: ResMut<State<AppQr>>,
    mut qrcode_str: ResMut<CurrentQrString>,
) {
    polling_counter.c += 1;
    // Every 100 clicks it tries triggers this system
    if polling_counter.c % polling_counter.poll_trigger == 0 {
        info!(
            "trying to receive data, polling at tick: {}",
            polling_counter.c
        );

        let r_height = height_channel.rx.try_recv();
        let r_invoice = invoice_channel.rx.try_recv();
        // let r_invoice_check = invoice_check_channel.rx.try_recv();
        // let r_move_player = player_move_channel.rx.try_recv();

        match r_height {
            Ok(r) => {
                info!("received response height: {}", r);
                let r_height_serialized = serde_json::from_str::<BlockData>(&r);
                match r_height_serialized {
                    Ok(o) => {
                        block_height_query.get_single_mut().unwrap().sections[0].value =
                            format!("Current Blockheight: {}", o.height);
                    }
                    Err(e) => {
                        info!("waiting to receive new block data: {}", e);
                    }
                };
                r
            }
            Err(e) => e.to_string(),
        };
        match r_invoice {
            Ok(r) => {
                info!("received response invoice: {}", r);
                let r_invoice_result = serde_json::from_str::<InvoiceData>(&r);
                match r_invoice_result {
                    Ok(o) => {
                        qrcode_str.0 = o.invoice.to_ascii_uppercase();
                        qr_state.set(AppQr::Fifty).unwrap();
                    }
                    Err(e) => {
                        info!("no new invoice data to get: {}", e);
                    }
                };
                r
            }
            Err(e) => e.to_string(),
        };
    }
}

#[allow(unused)]
pub fn api_height_sender(
    comm_channels: ResMut<HeightChannel>,
    mut count: ResMut<Api5000Sender>,
    actix_sever: Res<ActixServerURI>,
) {
    count.c += 1;
    // Every 100 clicks it tries triggers this system
    if count.c % count.poll_trigger == 0 {
        let pool = IoTaskPool::get();
        let cc = comm_channels.tx.clone();
        let server = actix_sever.clone().0;
        let _task = pool.spawn(async move {
            let api_response_text = reqwest::get(format!("{}/blockheight", server))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            cc.try_send(api_response_text);
        });
    };
}
