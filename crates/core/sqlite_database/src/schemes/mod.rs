use std::fmt::Debug;

use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
    log::info,
};
use general::AsyncChannel;

use crate::{database_traits::DatabaseSql, ConnectionSchema, Database};

pub mod auth_server;
pub mod game_server;

pub trait DatabaseSchemeAppExtension {
    fn register_sql_action<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(&mut self);
}

impl DatabaseSchemeAppExtension for App {
    fn register_sql_action<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(&mut self) {
        self.insert_resource(AsyncChannel::<T>::new());
        self.add_systems(Update, (read_channel::<T>, execute_sql::<T>));
    }
}

fn read_channel<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(
    channel: Res<AsyncChannel<T>>,
    mut commands: Commands,
) {
    if let Ok(channel) = channel.reciever_channel.try_lock() {
        for new_update_row in channel.iter() {
            commands.spawn(new_update_row);
        }
    }
}

fn execute_sql<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(
    database: Res<Database>,
    pending_data: Query<(Entity, &T)>,
    mut commands: Commands,
) {
    if pending_data.is_empty() {
        return;
    }
    let Ok(mut connection) = database.connection.lock() else {
        return;
    };

    for (entity, update_row) in pending_data.iter() {
        let Ok(tx) = connection.transaction() else {
            continue;
        };
        if let Some(sql) = update_row.to_sql() {
            let _ = tx.execute_schema(sql);
        } else {
            info!("Failed to convert UpdateRow to sql: {:?}", update_row)
        }
        let Ok(_) = tx.commit() else {
            continue;
        };
        commands.entity(entity).despawn_recursive();
    }
}
