use std::sync::Arc;

use arts_core::http_server::TideServerResource;
use bevy::{ecs::world::Mut, prelude::Plugin};

use crate::database::Database;

use self::{
    requests::{Logout, SignIn, SignUp},
    supabase::SupabaseConnection,
};

pub mod requests;
pub mod supabase;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SupabaseConnection::new(None, None));
        app.world.resource_scope(|world, database: Mut<Database>| {
            world.resource_scope(|world, mut tide: Mut<TideServerResource>| {
                let supabase = world.get_resource::<SupabaseConnection>().unwrap();
                tide.0.at("/auth/signin").post(SignIn {
                    supabase: Arc::new(supabase.clone()),
                    database: database.clone(),
                });
                tide.0.at("/auth/signout").post(Logout {
                    supabase: Arc::new(supabase.clone()),
                });
                tide.0.at("/auth/signup").post(SignUp {
                    supabase: Arc::new(supabase.clone()),
                });
            });
        });
    }
}
