use rusqlite::{params, Connection, Result};

use crate::entity::{checkerboard::CheckerBoard, user::User};

// 初始化数据库
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("minesweeper.db");
    match conn {
        Ok(conn) => {
            let result1 = conn.execute(
                "create table if not exists user (
                    id integer primary key not null,
                    name text not null unique,
                    password text not null
                )",
                [],
            );
            match result1 {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
            let result2 = conn.execute(
                "create table if not exists checker_board (
                    id integer primary key not null,
                    user_id integer not null references user(id),
                    date text not null,
                    rows integer not null,
                    cols integer not null,
                    is_win bool not null,
                    mine_count integer not null
                )",
                [],
            );
            match result2 {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
            Ok(conn)
        }
        Err(err) => Err(err),
    }
}

// 插入user,插入成功返回受影响的行数，否则返回失败原因
pub fn insert_user(user: &User) -> Result<usize> {
    let conn = Connection::open("minesweeper.db")?;
    conn.execute(
        "insert into user (name,password) values (?1,?2);",
        params![user.name, user.password],
    )
}

// 更具user_name查询user
pub fn select_user_by_name(user_name: &String) -> Result<User> {
    let conn = Connection::open("minesweeper.db")?;
    conn.query_row(
        "select * from user where name=?1",
        params![user_name],
        |row| Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?)),
    )
}

// 更具user_id查询user
pub fn select_user_by_id(user_id: i32) -> Result<User> {
    let conn = Connection::open("minesweeper.db")?;
    conn.query_row("select * from user where id=?1", params![user_id], |row| {
        Ok(User::new(row.get(0)?, row.get(1)?, row.get(2)?))
    })
}

// 保存游戏记录
pub fn save_checker_board(checker: &CheckerBoard) {
    let conn = Connection::open("minesweeper.db").unwrap();
    let _result = conn.execute(
        "insert into checker_board (user_id,date,rows,cols,is_win,mine_count) values (?1,?2,?3,?4,?5,?6)",
        params![
            checker.user_id,
            checker.date,
            checker.rows,
            checker.cols,
            checker.is_win,
            checker.mine_count
        ],
    );
}

// 查找游戏记录，通过用户ID
pub fn select_checker_board_by_user_id(user_id: i32) -> Vec<CheckerBoard> {
    let conn = Connection::open("minesweeper.db").unwrap();
    let mut stmt = conn
        .prepare("select * from checker_board where user_id=:user_id;")
        .unwrap();
    let checkers = stmt
        .query_map([user_id], |row| {
            Ok(CheckerBoard {
                id: row.get(0)?,
                user_id: row.get(1)?,
                date: row.get(2)?,
                rows: row.get(3)?,
                cols: row.get(4)?,
                is_win: row.get(5)?,
                mine_count: row.get(6)?,
            })
        })
        .unwrap();
    let mut checker_vec = Vec::new();
    for ele in checkers {
        match ele {
            Ok(checker) => checker_vec.push(checker),
            Err(_) => {}
        }
    }
    checker_vec
}

// 删除游戏记录
pub fn delete_record(record_id: i32) -> Result<i32> {
    let conn = Connection::open("minesweeper.db").unwrap();
    let result = conn.execute(
        "delete from checker_board where id=?1",
        params![record_id],
    );
    match result {
        Ok(_) => Ok(record_id),
        Err(err) => Err(err),
    }
}
