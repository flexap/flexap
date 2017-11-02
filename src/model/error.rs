use diesel;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error
{
    DieselResultError(diesel::result::Error),
    DieselConnectionError(diesel::ConnectionError)
}

impl From<diesel::result::Error> for Error
{
    fn from(error: diesel::result::Error) -> Self
    {
        Error::DieselResultError(error)
    }
}

impl From<diesel::ConnectionError> for Error
{
    fn from(error: diesel::ConnectionError) -> Self
    {
        Error::DieselConnectionError(error)
    }
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Error::DieselResultError(ref error) => error.fmt(f),
            Error::DieselConnectionError(ref error) => error.fmt(f),
        }
    }
}

impl error::Error for Error
{
    fn description(&self) -> &str
    {
        match *self {
            Error::DieselResultError(ref error) => error.description(),
            Error::DieselConnectionError(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::DieselResultError(ref error) => Some(error),
            Error::DieselConnectionError(ref error) => Some(error),
        }
    }
}
