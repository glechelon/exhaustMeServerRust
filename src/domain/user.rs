use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub identifier: String,
    pub name: String,
}
