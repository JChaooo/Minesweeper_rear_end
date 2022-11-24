use rusqlite::{params, Connection, Result};

use crate::entity::user::User;

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
                    time text not null,
                    row_count integer not null,
                    column_count integer not null,
                    mines_and_index text not null
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
