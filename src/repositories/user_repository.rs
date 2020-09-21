use crate::domain::user::User;
use sqlx::mysql::MySqlDone;
use sqlx::{query, query_as, Error, MySql, Pool};

pub async fn get_user(db: &Pool<MySql>, id: &str) -> Result<User, Error> {
    query_as::<_, User>("SELECT * FROM USERS WHERE identifier = ?")
        .bind(id)
        .fetch_one(db)
        .await
}

pub async fn get_users(db: &Pool<MySql>) -> Result<Vec<User>, Error> {
    query_as::<_, User>("SELECT * FROM USERS")
        .fetch_all(db)
        .await
}

pub async fn create_user(db: &Pool<MySql>, user: &User) -> Result<MySqlDone, Error> {
    query("INSERT INTO USERS(identifier, name) VALUES(?, ?)")
        .bind(&user.identifier)
        .bind(&user.name)
        .execute(db)
        .await
}

pub async fn update_user(db: &Pool<MySql>, id: &str, user: &User) -> Result<MySqlDone, Error> {
    query("UPDATE USERS SET name = ? WHERE identifier = ?")
        .bind(&user.name)
        .bind(&id)
        .execute(db)
        .await
}

pub async fn delete_user(db: &Pool<MySql>, id: &str) -> Result<MySqlDone, Error> {
    query("DELETE FROM USERS WHERE identifier = ?")
        .bind(&id)
        .execute(db)
        .await
}
