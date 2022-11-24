// 这是棋盘实体

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MinesIndexType = HashMap<i32, [i32; 2]>;
//time: 棋盘生成时间
// row_count：棋盘x轴数量
// column_count：棋盘y轴数量
// mines_and_index：炸弹集合，及其在棋盘中的位置
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckerBoard {
    id: i32,
    user_id:i32,
    time: String,
    row_count: i32,
    column_count: i32,
    mines_and_index: MinesIndexType,
}

impl CheckerBoard {
    pub fn new(
        id: i32,
        user_id:i32,
        time: String,
        row_count: i32,
        column_count: i32,
        mines_and_index: MinesIndexType,
    ) -> Self {
        CheckerBoard {
            id,
            user_id,
            time,
            row_count,
            column_count,
            mines_and_index,
        }
    }
}
