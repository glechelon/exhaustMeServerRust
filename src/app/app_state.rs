use crate::users::users_state::{UserState};

#[derive(Clone)]    
pub struct AppState {
   pub user_state: UserState,
}