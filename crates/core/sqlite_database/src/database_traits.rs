use general::game_meta::GameId;

/// A trait that all data submitted into the database must implement
pub trait DatabaseData {
    /// Converts this data into itself
    fn to_database_string(&self) -> Option<String>;

    /// A unique name of the data that is used for the database column and registration
    fn column_name(&self) -> &str;

    fn to_database_data(&self) -> Option<PureDatabaseData> {
        let Some(data) = self.to_database_string() else {
            return None;
        };
        Some(PureDatabaseData {
            column_name: self.column_name().to_string(),
            data,
        })
    }
}

/// DatabaseTable that does not require a [`GameId`]
pub trait DatabaseTable {
    fn table_name(&self) -> String;
}

/// DatabaseTable that does require a [`GameId`]
pub trait GameDatabaseTable {
    fn table_name(&self, game_id: &GameId) -> String;
}

/// A struct used to represent literal DatabaseData in its pure form. The data in here is copied straight into the database
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
pub struct PureDatabaseData {
    /// The data formatted into how it should look in the database
    pub data: String,
    /// The column name of the data
    pub column_name: String,
}

impl DatabaseData for PureDatabaseData {
    fn to_database_string(&self) -> Option<String> {
        Some(self.data.clone())
    }

    fn column_name(&self) -> &str {
        &self.column_name
    }
}

/// A trait implemented on structs that represent sql commands
pub trait DatabaseSql {
    fn to_sql(&self) -> Option<(String, Vec<String>)>;
}
