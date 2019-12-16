use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result as DResult};
use diesel::serialize::{Output, Result as SResult, ToSql};
use diesel::sql_types::Integer;

use std::io::Write;

#[repr(i32)]
#[derive(Copy, Clone, Debug, FromSqlRow, AsExpression)]
pub enum RoleType {
    Tag = 0,
    Silence = 1,
    User = 2,
    Moderator = 3,
    Admin = 4,
}

impl<DB> ToSql<Integer, DB> for RoleType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> SResult {
        (*self as i32).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for RoleType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> DResult<Self> {
        Ok(match i32::from_sql(bytes)? {
            0 => RoleType::Tag,
            1 => RoleType::Silence,
            2 => RoleType::User,
            3 => RoleType::Moderator,
            4 => RoleType::Admin,
            x => return Err(format!("Unrecognized join status {}", x).into()),
        })
    }
}
