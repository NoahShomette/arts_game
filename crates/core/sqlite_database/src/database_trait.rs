/// A trait that all data submitted into the database must implement
pub trait DatabaseData {
    /// Converts this data into itself
    fn to_database_data(&self) -> String;

    /// A unique name of the data that is used for the database column and registration
    fn name(&self) -> &str;
}
