// 这是游戏记录实体

use super::{checkerboard::CheckerBoard, user::User};
use serde::{Deserialize, Serialize};

// user:记录所属的用户
// history:所有的记录
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    user: User,
    history: Vec<CheckerBoard>,
}


impl Record {
    pub fn new(user: User, history: Vec<CheckerBoard>) -> Self {
        Record { user, history }
    }
}
