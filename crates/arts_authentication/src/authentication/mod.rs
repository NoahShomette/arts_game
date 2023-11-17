use std::sync::Arc;

use bevy::{ecs::world::Mut, prelude::Plugin};

use crate::TideServerResource;

use self::{
    requests::{Logout, SignIn},
    supabase::SupabaseConnection,
};

pub mod requests;
pub mod supabase;

pub struct AuthenticationPlugin;

impl Plugin for AuthenticationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SupabaseConnection::new(None, None));
        app.world
            .resource_scope(|world, mut tide: Mut<TideServerResource>| {
                let supabase = world.get_resource::<SupabaseConnection>().unwrap();
                tide.0.at("/auth/sign_in").post(SignIn {
                    supabase: Arc::new(supabase.clone()),
                });
                tide.0.at("/auth/logout").post(Logout {
                    supabase: Arc::new(supabase.clone()),
                });
            });
    }
}
