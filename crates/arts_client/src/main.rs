use arts_client::ReqwestClient;
use arts_core::async_runners;
use arts_core::client_authentication::Password;
use bevy::app::App;
use bevy::prelude::{Res, Resource, Startup, Update};
use bevy::tasks::{TaskPool, TaskPoolBuilder};
use bevy::DefaultPlugins;

fn main() {
    let mut app = App::new();
    app.insert_resource(ReqwestClient::new());
    app.add_plugins(DefaultPlugins);
    app.insert_resource(TaskPoolRes(TaskPoolBuilder::new().num_threads(2).build()));
    app.add_systems(Startup, sign_in);
    app.add_systems(Update, receive_auth_results);
    app.run();
}

#[derive(Resource)]
pub struct TaskPoolRes(pub TaskPool);

fn sign_in(supabase: Res<ReqwestClient>, task_pool_res: Res<TaskPoolRes>) {
    let supabase = supabase.clone();
    async_runners::run_async(
        async move {
            let result = supabase
                .client
                .get("http://127.0.0.1:2030/auth/sign_in")
                .send_json(Password {
                    email: "noahshomette@gmail.com".to_string(),
                    password: "123456789".to_string(),
                });
            match result {
                Ok(response) => {
                    let _ = supabase.sender_channel.send(Ok(response));
                }
                Err(err) => {
                    let _ = supabase.sender_channel.send(Err(format!("{}", err)));
                }
            };
        },
        &task_pool_res.0,
    )
    .unwrap()
    .detach();
}

fn receive_auth_results(supa: Res<ReqwestClient>) {
    let Ok(channel) = supa.reciever_channel.lock() else {
        return;
    };

    let result = channel.try_recv();

    if result.is_ok() {
        #[cfg(not(target_arch = "wasm32"))]
        println!("{:?}", result.unwrap().unwrap().into_string());

        #[cfg(target_arch = "wasm32")]
        {
            let js: wasm_bindgen::JsValue = format!("{:?}", result.unwrap()).into();
            web_sys::console::log_1(&js);
        }
    };
}
