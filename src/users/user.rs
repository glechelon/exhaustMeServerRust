use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32, //TODO: remplacer par un uuid
    pub name: String,
}
