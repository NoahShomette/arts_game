use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
};

use crate::{database_trait::DatabaseData, ConnectionSchema, Database};

#[derive(Component)]
pub struct NewRow {
    pub table_name: String,
    pub row_id: Box<dyn DatabaseData + Send + Sync>,
    pub database_data: Vec<Box<dyn DatabaseData + Send + Sync>>,
}

impl NewRow {
    pub fn to_sql(&self) -> (String, Vec<String>) {
        let params_vec: Vec<String> = self
            .database_data
            .iter()
            .map(|x| x.to_database_data())
            .collect();
        //            "UPDATE games_meta SET pending_players = ?1, WHERE game_id = ?2",

        let mut sql_command = format!("UPDATE {} SET ", self.table_name);
        for (index, data) in self.database_data.iter().enumerate() {
            sql_command.push_str(&format!("{} = ?{}, ", data.name(), index + 1));
        }
        sql_command.push_str(&format!(
            "WHERE {} = {}, ",
            self.row_id.name(),
            self.row_id.to_database_data()
        ));
        (sql_command, params_vec)
    }
}

#[derive(Component)]
pub struct UpdateRow {
    pub table_name: String,
    pub row_id: Box<dyn DatabaseData + Send + Sync>,
    pub database_data: Vec<Box<dyn DatabaseData + Send + Sync>>,
}

impl UpdateRow {
    pub fn to_sql(&self) -> (String, Vec<String>) {
        let params_vec: Vec<String> = self
            .database_data
            .iter()
            .map(|x| x.to_database_data())
            .collect();

        let mut sql_command = format!("UPDATE {} SET ", self.table_name);
        for (index, data) in self.database_data.iter().enumerate() {
            sql_command.push_str(&format!("{} = ?{}, ", data.name(), index + 1));
        }
        sql_command.push_str(&format!(
            "WHERE {} = {}, ",
            self.row_id.name(),
            self.row_id.to_database_data()
        ));
        (sql_command, params_vec)
    }
}

fn update_rows(
    database: Res<Database>,
    pending_data: Query<(Entity, &UpdateRow)>,
    mut commands: Commands,
) {
    if pending_data.is_empty() {
        return;
    }
    let Ok(mut connection) = database.connection.lock() else {
        return;
    };

    for (entity, new_game) in pending_data.iter() {
        let Ok(tx) = connection.transaction() else {
            continue;
        };
        let _ = tx.execute_schema(new_game.to_sql());
        let Ok(_) = tx.commit() else {
            continue;
        };
        commands.entity(entity).despawn_recursive();
    }
}

mod test {}
