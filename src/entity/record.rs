// 这是游戏记录实体

use super::{checkerboard::CheckerBoard, user::User};
use serde::{Deserialize, Serialize};

// user:记录所属的用户
// records:所有的记录
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    user: User,
    records: History,
}

// time:记录的时间
// checker_board:单条记录的游戏数据（棋盘以及炸弹位置）
#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    time: String,
    checker_board: CheckerBoard,
}

impl Record {
    pub fn new(user: User, records: History) -> Self {
        Record { user, records }
    }
}

impl History {
    pub fn new(time: String, checker_board: CheckerBoard) -> Self {
        History {
            time,
            checker_board,
        }
    }
}
