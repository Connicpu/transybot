use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result as DResult};
use diesel::serialize::{Output, Result as SResult, ToSql};
use diesel::sql_types::Integer;

use std::io::Write;

#[repr(i32)]
#[derive(Copy, Clone, Debug, FromSqlRow, AsExpression)]
pub enum ResponseType {
    Rule = 0,
    TrueOrFalse = 1,
    MultipleChoice = 2,
    OpenEnded = 3,
    Pronoun = 4,
    Role = 5,
}

impl<DB> ToSql<Integer, DB> for ResponseType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> SResult {
        (*self as i32).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for ResponseType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> DResult<Self> {
        Ok(match i32::from_sql(bytes)? {
            0 => ResponseType::Rule,
            1 => ResponseType::TrueOrFalse,
            2 => ResponseType::MultipleChoice,
            3 => ResponseType::OpenEnded,
            4 => ResponseType::Pronoun,
            5 => ResponseType::Role,
            x => return Err(format!("Unrecognized response type {}", x).into()),
        })
    }
}
