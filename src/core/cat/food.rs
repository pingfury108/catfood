use diesel::prelude::{Queryable, Selectable};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = super::schema::cat_food)]
pub struct Food {
    pub id: i32,
    pub gid: String,
    pub title: String,
    pub describe: String,
}
