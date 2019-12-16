#[derive(Queryable)]
pub struct User {
    pub user_id: i64,
    pub guild_id: i64,
    pub username: String,
    
    pub server_rank: i32,
    pub rank_role: Option<i64>,
    pub join_status: i32,

    pub silenced_until: Option<chrono::NaiveDateTime>,

    pub first_joined: Option<chrono::NaiveDateTime>,
    pub last_left: Option<chrono::NaiveDateTime>,
    pub times_left: i32,

    pub confirmed_18: bool,
}
