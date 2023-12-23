use std::fmt::Debug;

use bevy::ecs::component::Component;

use crate::database_traits::{DatabaseData, DatabaseSql, PureDatabaseData};

#[derive(Component, Clone)]
pub struct UpdateRow {
    pub table_name: String,
    pub row_id: PureDatabaseData,
    pub database_data: Vec<PureDatabaseData>,
}

impl UpdateRow {
    /// Constructs a new [`UpdateRow`] struct if the data contained is valid
    pub fn new(
        table_name: String,
        row_id: &impl DatabaseData,
        database_data: &impl DatabaseData,
    ) -> Result<UpdateRow, String> {
        let Some(row_id) = row_id.to_database_data() else {
            return Err("Row Id invalid".to_string());
        };

        let Some(database_data) = database_data.to_database_data() else {
            return Err("DatabaseData invalid".to_string());
        };

        Ok(UpdateRow {
            table_name,
            row_id,
            database_data: vec![database_data],
        })
    }
}

impl Debug for UpdateRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateRow")
            .field("table_name", &self.table_name)
            .field("row_id", &self.row_id.column_name())
            .field(
                "database_data",
                &self
                    .database_data
                    .iter()
                    .map(|x| x.column_name())
                    .collect::<Vec<&str>>(),
            )
            .finish()
    }
}

impl DatabaseSql for UpdateRow {
    fn to_sql(&self) -> Option<(String, Vec<String>)> {
        let params_vec: Vec<String> = self
            .database_data
            .iter()
            .filter_map(|x| x.to_database_string())
            .collect();
        let mut sql_command = format!("UPDATE \"{}\" SET ", self.table_name);
        for (index, data) in self.database_data.iter().enumerate() {
            sql_command.push_str(&format!("{} = ?{}, ", data.column_name(), index + 1));
        }
        let Some(row_id_data) = self.row_id.to_database_string() else {
            return None;
        };
        sql_command.push_str(&format!(
            "WHERE {} = {}, ",
            self.row_id.column_name(),
            row_id_data
        ));
        Some((sql_command, params_vec))
    }
}
