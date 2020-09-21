use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}
