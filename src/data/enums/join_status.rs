use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result as DResult};
use diesel::serialize::{Output, Result as SResult, ToSql};
use diesel::sql_types::Integer;

use std::io::Write;

#[repr(i32)]
#[derive(Copy, Clone, Debug, FromSqlRow, AsExpression)]
pub enum JoinStatus {
    FreshUser = 0,
    Answering = 1,
    Finished = 2,
    Accepted = 3,
}

impl<DB> ToSql<Integer, DB> for JoinStatus
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> SResult {
        (*self as i32).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for JoinStatus
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> DResult<Self> {
        Ok(match i32::from_sql(bytes)? {
            0 => JoinStatus::FreshUser,
            1 => JoinStatus::Answering,
            2 => JoinStatus::Finished,
            3 => JoinStatus::Accepted,
            x => return Err(format!("Unrecognized join status {}", x).into()),
        })
    }
}
