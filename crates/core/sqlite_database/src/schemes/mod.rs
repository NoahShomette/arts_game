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
use general::{create_async_channel, AsyncChannelReceiver};

use crate::{database_traits::DatabaseSql, ConnectionSchema, Database};

pub mod auth_server;
pub mod game_server;

pub trait DatabaseSchemeAppExtension {
    fn server_register_sql_action<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(
        &mut self,
    );
}

impl DatabaseSchemeAppExtension for App {
    fn server_register_sql_action<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(
        &mut self,
    ) {
        let (async_receiver, async_sender) = create_async_channel::<T>();
        self.insert_resource(async_receiver);
        self.insert_resource(async_sender);

        self.add_systems(Update, (read_channel::<T>, execute_sql::<T>));
    }
}

fn read_channel<T: Send + Sync + 'static + Component + DatabaseSql + Debug>(
    channel: Res<AsyncChannelReceiver<T>>,
    mut commands: Commands,
) {
    if let Ok(channel) = channel.reciever_channel.try_lock() {
        while let Ok(new_message) = channel.try_recv() {
            commands.spawn(new_message);
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
    let Some(mut connection) = database.connection.try_lock() else {
        return;
    };

    for (entity, update_row) in pending_data.iter() {
        let Ok(tx) = connection.transaction() else {
            continue;
        };
        if let Some(sql) = update_row.to_sql() {
            match tx.execute_schema(sql) {
                Ok(_) => {}
                Err(err) => println!("Failed to execute SQL: {}", err),
            }
        } else {
            info!("Failed to convert UpdateRow to sql: {:?}", update_row)
        }
        let Ok(_) = tx.commit() else {
            continue;
        };
        commands.entity(entity).despawn_recursive();
    }
}
