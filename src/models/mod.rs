use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::mesaje)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Mesaj {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDate>,
}
