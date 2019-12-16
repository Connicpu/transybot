use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result as DResult};
use diesel::serialize::{Output, Result as SResult, ToSql};
use diesel::sql_types::Integer;

use std::io::Write;

#[repr(i32)]
#[derive(Copy, Clone, Debug, FromSqlRow, AsExpression)]
pub enum PermissionType {
    GrantRead = 0,
    GrantWrite = 1,
    DenyRead = 2,
    DenyWrite = 3,
}

impl<DB> ToSql<Integer, DB> for PermissionType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> SResult {
        (*self as i32).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for PermissionType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> DResult<Self> {
        Ok(match i32::from_sql(bytes)? {
            0 => PermissionType::GrantRead,
            1 => PermissionType::GrantWrite,
            2 => PermissionType::DenyRead,
            3 => PermissionType::DenyWrite,
            x => return Err(format!("Unrecognized permission type {}", x).into()),
        })
    }
}
