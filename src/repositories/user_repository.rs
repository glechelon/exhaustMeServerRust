
use sqlx::Pool;
use sqlx::MySql;

use crate::domain::user::User;


#[derive(Clone)]
pub struct UserRepository {
    pub db : Pool<MySql>
}


impl UserRepository {

    pub async fn get_user(&self, id: &String) -> Result<User, sqlx::Error> {
        return sqlx::query_as::<_, User>("SELECT * FROM USERS WHERE identifier = ?").bind(id).fetch_one(&self.db).await;
    }

    
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        return sqlx::query_as::<_, User>("SELECT * FROM USERS").fetch_all(&self.db).await;
    }

}
