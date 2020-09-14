use crate::domain::user::User;
use sqlx::mysql::MySqlDone;
use sqlx::{query, query_as, Error, MySql, Pool};

#[derive(Clone)]
pub struct UserRepository {
    pub db: Pool<MySql>,
}

impl UserRepository {
    pub async fn get_user(&self, id: &str) -> Result<User, Error> {
        query_as::<_, User>("SELECT * FROM USERS WHERE identifier = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        query_as::<_, User>("SELECT * FROM USERS")
            .fetch_all(&self.db)
            .await
    }

    pub async fn create_user(&self, user: &User) -> Result<MySqlDone, Error> {
        query("INSERT INTO USERS(identifier, name) VALUES(?, ?)")
            .bind(&user.identifier)
            .bind(&user.name)
            .execute(&self.db)
            .await
    }

    pub async fn update_user(&self, id: &str, user: &User) -> Result<MySqlDone, Error> {
        query("UPDATE USERS SET name = ? WHERE identifier = ?")
            .bind(&user.name)
            .bind(&id)
            .execute(&self.db)
            .await
    }

    pub async fn delete_user(&self, id: &str) -> Result<MySqlDone, Error> {
        query("DELETE FROM USERS WHERE identifier = ?")
            .bind(&id)
            .execute(&self.db)
            .await
    }
}
