use crate::schema;
use anyhow::Result;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::{Insertable, QueryDsl, Queryable, Selectable};
use diesel::{AsChangeset, ExpressionMethods, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Queryable, Selectable, Serialize, Deserialize, Insertable, Clone, AsChangeset,
)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub uid: String,
    pub name: String,
    pub pwd: String,
    pub display_name: String,
    pub email: Option<String>,
}

pub async fn user_one(pool: &Pool, id: String) -> Result<User, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res: Vec<User> = conn
        .interact(move |conn| {
            use schema::users::dsl::*;
            users.filter(uid.eq(&id[..])).load(conn)
        })
        .await??;
    if !res.is_empty() {
        return Ok(res[0].clone());
    }

    Ok(User::default())
}

pub async fn user_one_by_name(
    pool: &Pool,
    uname: String,
) -> Result<User, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res: Vec<User> = conn
        .interact(move |conn| {
            use schema::users::dsl::*;
            users.filter(name.eq(&uname[..])).load(conn)
        })
        .await??;
    if !res.is_empty() {
        return Ok(res[0].clone());
    }

    Ok(User::default())
}

pub async fn user_new(pool: &Pool, user: User) -> Result<User, Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let res: User = conn
        .interact(move |conn| {
            diesel::insert_into(schema::users::table)
                .values(user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await??;
    Ok(res)
}

pub async fn food_save(pool: &Pool, user: User) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    let _ = conn
        .interact(move |conn| {
            use schema::users::dsl::*;
            diesel::update(schema::users::table)
                .filter(uid.eq(&user.uid.clone()[..]))
                .set(user)
                .execute(conn)
        })
        .await??;
    Ok(())
}
