use super::schema::ping_history;

#[derive(Queryable)]
pub struct PingHistoryItem {
    pub id: i64,
    pub url: String,
    pub iteration: i64,
    pub result: i64,
}

#[derive(Insertable)]
#[table_name = "ping_history"]
pub struct NewPingHistoryItem<'a> {
    pub url : &'a str,
    pub iteration: &'a i32 ,
    pub result: &'a i32,
}