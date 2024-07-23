use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::cat_food)]
pub struct Food {
    pub gid: String,
    pub title: String,
    pub describe: String,
}
