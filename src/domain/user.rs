use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub identifier : String,
    pub name: String
}
