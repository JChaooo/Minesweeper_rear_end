// 这是棋盘实体

use actix_web::web;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::database;

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckerBoard {
    pub id: i32,
    pub user_id: i32,
    pub date: String,
    pub rows: i32,
    pub cols: i32,
    pub is_win: bool,
    pub mine_count: i32,
}

impl CheckerBoard {
    // 保存游戏记录
    pub fn save(data: web::Json<Value>) {
        let checker: CheckerBoard = serde_json::from_value(json!(data)).unwrap();
        database::save_checker_board(&checker);
    }
}
