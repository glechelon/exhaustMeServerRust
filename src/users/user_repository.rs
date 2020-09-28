use super::user::User;
use sqlx::mysql::MySqlDone;
use sqlx::{query, query_as, Error, MySql, Pool};

#[derive(Clone)]
pub struct UserRepository{
    pub db_pool : Box<Pool<MySql>>
}

impl  UserRepository{

pub async fn get_user(&self, id: &str) -> Result<User, Error> {
    query_as::<_, User>("SELECT * FROM USERS WHERE ID = ?")
        .bind(id)
        .fetch_one(&*self.db_pool)
        .await
}

pub async fn get_users(&self) -> Result<Vec<User>, Error> {
    query_as::<_, User>("SELECT * FROM USERS")
        .fetch_all(&*self.db_pool)
        .await
}

pub async fn create_user(&self, user: &User) -> Result<MySqlDone, Error> {
    query("INSERT INTO USERS( name) VALUES( ?)")
        .bind(&user.name)
        .execute(&*self.db_pool)
        .await
}

pub async fn update_user(&self, id: &i64, user: &User) -> Result<MySqlDone, Error> {
    query("UPDATE USERS SET name = ? WHERE ID = ?")
        .bind(&user.name)
        .bind(&id)
        .execute(&*self.db_pool)
        .await
}

pub async fn delete_user(&self,id: &i64) -> Result<MySqlDone, Error> {
    query("DELETE FROM USERS WHERE ID = ?")
        .bind(&id)
        .execute(&*self.db_pool)
        .await
}
}