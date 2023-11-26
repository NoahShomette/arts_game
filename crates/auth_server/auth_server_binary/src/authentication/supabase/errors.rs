use std::fmt::Display;

use thiserror::Error;

#[derive(Debug)]
pub enum AuthOk {
    SignUp(String),
    SignIn(String),
    RefreshToken(String),
    SignOut(String),
    Ok(String),
}

#[derive(Clone, Debug, Error)]
pub enum AuthErrors {
    Basic(String),
}

impl Display for AuthErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthErrors::Basic(string) => f.write_fmt(format_args!("Basic Error: {}", string)),
        }
    }
}

impl From<tide::Error> for AuthErrors {
    fn from(value: tide::Error) -> Self {
        Self::Basic(value.to_string())
    }
}
