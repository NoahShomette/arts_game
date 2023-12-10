use std::sync::Arc;

use self::{
    requests::{AuthenticateUser, RefreshTokenEndpoint, SignIn, SignOut, SignUp},
    supabase::SupabaseConnection,
};
use bevy::{ecs::world::Mut, prelude::Plugin};
use core_library::{http_server::TideServerResource, sqlite_database::Database};

pub mod requests;
pub mod supabase;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SupabaseConnection::new(None, None));
        app.world.resource_scope(|world, database: Mut<Database>| {
            world.resource_scope(|world, mut tide: Mut<TideServerResource>| {
                let supabase = world.get_resource::<SupabaseConnection>().unwrap();
                tide.0.at("/auth/sign_in").post(SignIn {
                    supabase: Arc::new(supabase.clone()),
                    database: database.clone(),
                });
                tide.0.at("/auth/sign_out").post(SignOut {
                    supabase: Arc::new(supabase.clone()),
                });
                tide.0.at("/auth/sign_up").post(SignUp {
                    supabase: Arc::new(supabase.clone()),
                });
                tide.0.at("/auth/refresh_token").post(RefreshTokenEndpoint {
                    supabase: Arc::new(supabase.clone()),
                });
                tide.0.at("/auth/authenticate_user").post(AuthenticateUser {
                    supabase: Arc::new(supabase.clone()),
                });
            });
        });
    }
}
