use std::sync::Arc;

use arts_core::http_server::TideServerResource;
use bevy::{app::Plugin, ecs::world::Mut};

use crate::{authentication::supabase::SupabaseConnection, database::Database};

use self::requests::RequestPlayerGames;

mod requests;

pub struct UserManagementPlugin;

impl Plugin for UserManagementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.resource_scope(|world, database: Mut<Database>| {
            world.resource_scope(|world, mut tide: Mut<TideServerResource>| {
                let supabase = world.get_resource::<SupabaseConnection>().unwrap();
                tide.0.at("/player/player_games").get(RequestPlayerGames {
                    supabase: Arc::new(supabase.clone()),
                    database: database.clone(),
                });
            });
        });
    }
}
