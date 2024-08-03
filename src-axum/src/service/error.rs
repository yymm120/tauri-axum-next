use serde::Serialize;

use serde_with::{serde_as, DisplayFromStr};
use crate::model;
pub type Result<T> = core::result::Result<T, Error>;

#[allow(unused)]
#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	ServiceError { entity: &'static str, id: i64 },
    Model(model::Error),
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error)
}
impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}
impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Self::Sqlx(val)
    }
}
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "service error! - {self:?}")
	}
}

impl std::error::Error for Error {}