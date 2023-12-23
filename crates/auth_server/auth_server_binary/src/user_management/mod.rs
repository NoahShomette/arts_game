use std::sync::Arc;

use bevy::{app::Plugin, ecs::world::Mut};
use core_library::{
    authentication::client_authentication::Claims, http_server::TideServerResource,
};
use tide::{Error, Request};

use crate::authentication::supabase::SupabaseConnection;
use core_library::sqlite_database::Database;

use self::requests::{RequestPlayerGames, SetPlayerUsername};

mod requests;

pub struct UserManagementPlugin;

impl Plugin for UserManagementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.resource_scope(|world, database: Mut<Database>| {
            world.resource_scope(|world, mut tide: Mut<TideServerResource>| {
                let supabase = world
                    .get_resource::<SupabaseConnection>()
                    .expect("SupabaseConnection must be in world when launching TideServer");
                tide.0.at("/player/player_games").get(RequestPlayerGames {
                    supabase: Arc::new(supabase.clone()),
                    database: database.clone(),
                });
                tide.0.at("/player/set_username").post(SetPlayerUsername {
                    supabase: Arc::new(supabase.clone()),
                    database: database.clone(),
                });
            });
        });
    }
}

/// Verifies a JWT as being a valid signed JWT from Supabase. Will decode it and return [`Claims`].
///
/// Returns an error if it fails at any part
pub fn verify_decode_jwt<T>(
    req: &Request<T>,
    supabase: &SupabaseConnection,
) -> Result<Claims, Error> {
    let Some(access_token) = req.header("authorization") else {
        return Err(Error::from_str(400, "No Authorization Bearer found"));
    };
    let string_at = access_token.to_string();
    let access_token = string_at.split_whitespace().collect::<Vec<&str>>()[1];
    let (access_token, _) = access_token.split_at(access_token.len() - 2);

    let Ok(claims) = supabase.jwt_valid(access_token) else {
        return Err(Error::from_str(403, "Invalid Access Token"));
    };
    Ok(claims)
}
