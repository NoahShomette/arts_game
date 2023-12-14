/// A trait that all data submitted into the database must implement
pub trait DatabaseData {
    /// Converts this data into itself
    fn to_database_string(&self) -> Option<String>;

    /// A unique name of the data that is used for the database column and registration
    fn column_name(&self) -> &str;
}

/// A struct used for DatabaseData that does not have its own struct. Eg integers and the like
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct AdHocDatabaseData {
    /// The data formatted into how it should look in the database
    pub data: String,
    /// The column name of the data
    pub column_name: String,
}

impl DatabaseData for AdHocDatabaseData {
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
