use std::sync::Arc;

use bevy::{ecs::world::Mut, prelude::Plugin};
use tide::utils::async_trait;
use tide::{Endpoint, Request};

use crate::TideServerResource;

use self::supabase::Supabase;

pub mod supabase;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Supabase::new(None, None, None));
        app.world
            .resource_scope(|world, mut tide: Mut<TideServerResource>| {
                let supabase = world.get_resource::<Supabase>().unwrap();
                tide.0.at("/auth/sign_in").post(SignIn {
                    supabase: Arc::new(supabase.clone()),
                });
            });
    }
}

struct SignIn {
    supabase: Arc<Supabase>,
}

#[async_trait]
impl Endpoint<()> for SignIn {
    async fn call(&self, req: Request<()>) -> tide::Result {
        sign_in(req, &self.supabase).await
    }
}

async fn sign_in(mut req: Request<()>, supabase: &Supabase) -> tide::Result {
    let result = supabase.sign_in_password(req.body_json().await?).await?;
    Ok(tide::Response::builder(200)
        .body(result.text().unwrap())
        .build())
}
