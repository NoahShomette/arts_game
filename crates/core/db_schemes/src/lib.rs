use rusqlite::{params_from_iter, Error, Transaction};

pub mod games_meta;
pub mod game_tables;

pub trait ConnectionSchema {
    /// Executes the desired schema. param.0 is the SQL command, param.1 are the params for the command
    fn execute_schema(&self, params: (String, Vec<String>)) -> Result<usize, Error>;
}

impl ConnectionSchema for Transaction<'_> {
    fn execute_schema(&self, params: (String, Vec<String>)) -> Result<usize, Error> {
        self.execute(&params.0, params_from_iter(params.1.iter()))
    }
}
