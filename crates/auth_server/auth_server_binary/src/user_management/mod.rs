use std::sync::Arc;

use core_library::{authentication::client_authentication::Claims, http_server::TideServerResource};
use bevy::{app::Plugin, ecs::world::Mut};
use tide::{Error, Request};

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

/// Verifies a JWT as being a valid signed JWT from Supabase. Will decode it and return [`Claims`].
///
/// Returns an error if it fails at any part
pub fn verify_decode_jwt(
    req: &Request<()>,
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

pub fn request_access_token(req: &Request<()>) -> Result<String, Error> {
    let Some(access_token) = req.header("authorization") else {
        return Err(Error::from_str(400, "No Authorization Bearer found"));
    };
    let string_at = access_token.to_string();
    Ok(string_at.split_whitespace().collect::<Vec<&str>>()[1].to_string())
}
