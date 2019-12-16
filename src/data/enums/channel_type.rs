use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result as DResult};
use diesel::serialize::{Output, Result as SResult, ToSql};
use diesel::sql_types::Integer;

use std::io::Write;

#[auto_enum::auto_enum(i32, checked)]
#[derive(FromSqlRow, AsExpression)]
pub enum ChannelType {
    Log = 0,
    JoinLog = 1,
    JoinApproval = 2,
    Rules = 3,
    Roles = 4,
    Deletion = 5,
}

impl<DB> ToSql<Integer, DB> for ChannelType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> SResult {
        (*self as i32).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for ChannelType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> DResult<Self> {
        Ok(match i32::from_sql(bytes)? {
            0 => ChannelType::Log,
            1 => ChannelType::JoinLog,
            2 => ChannelType::JoinApproval,
            3 => ChannelType::Rules,
            4 => ChannelType::Roles,
            5 => ChannelType::Deletion,
            x => return Err(format!("Unrecognized channel type {}", x).into()),
        })
    }
}
