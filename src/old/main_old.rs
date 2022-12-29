use std::future::Future;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool, time::FixedTimestep};
use bevy_inspector_egui::bevy_egui::{egui, EguiContext, EguiPlugin};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[bevy_main]
fn main() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_plugins(bevy_webgl2::DefaultPlugins);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins);

    app.add_plugin(EguiPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(request_data),
        )
        .add_system(fetch_data)
        .add_system(show)
        .init_resource::<Option<Data>>()
        .add_startup_system(setup_data_fetch);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_fullscreen::FullViewportPlugin);

    app.run();
}

#[derive(Debug, Resource)]
struct Data {
    pub text: String,
}

struct Worker<I, O> {
    input: UnboundedSender<I>,
    output: UnboundedReceiver<O>,
}

impl<I, O> Worker<I, O> {
    fn spawn<Func, Fut>(thread_pool: &Res<AsyncComputeTaskPool>, function: Func) -> Self
    where
        Func: FnOnce(UnboundedReceiver<I>, UnboundedSender<O>) -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        let (input_tx, input_rx) = unbounded_channel::<I>();
        let (output_tx, output_rx) = unbounded_channel::<O>();
        thread_pool
            .spawn_local(function(input_rx, output_tx))
            .detach();
        Worker {
            input: input_tx,
            output: output_rx,
        }
    }
}

type DataGetter = Worker<(), anyhow::Result<Data>>;

fn setup_data_fetch(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    let fetcher = DataGetter::spawn(&thread_pool, |mut fetch_requests, datas| async move {
        let url = "http://127.0.0.1:8000/";
        let mut i = 0;
        loop {
            if fetch_requests.recv().await.is_some() {
                while fetch_requests.try_recv().is_ok() {} // Empty out buffer

                let res = reqwest::get(url).await.unwrap().text().await.unwrap();

                datas
                    .send(Ok(Data {
                        text: format!("result: {} : {}", res, i).to_owned(),
                    }))
                    .unwrap();
                i += 1;
            }
        }
    });
    fetcher.input.send(()).unwrap();
    commands.insert_resource(fetcher);
}

fn request_data(data_fetcher: Option<ResMut<DataGetter>>) {
    if let Some(fetcher) = data_fetcher {
        fetcher.input.send(()).unwrap();
    }
}

fn fetch_data(mut commands: Commands, data_fetcher: Option<ResMut<DataGetter>>) {
    if let Some(mut fetcher) = data_fetcher {
        while let Ok(fetch_result) = fetcher.output.try_recv() {
            if let Ok(data) = fetch_result {
                commands.insert_resource(data);
            }
        }
    }
}

fn show(egui_ctx: ResMut<EguiContext>, data: Option<Res<Data>>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        if let Some(data) = data.as_ref() {
            ui.label(&data.text);
        }
    });
}
