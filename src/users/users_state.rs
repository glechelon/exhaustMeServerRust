use super::user_repository::{UserRepository};

#[derive(Clone)]
pub struct UserState {
    pub user_repository: UserRepository
}