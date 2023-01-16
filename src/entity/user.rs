use actix_web::web;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    database,
    result::{ResData, DATA},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn new(id: i32, name: String, password: String) -> Self {
        User { id, name, password }
    }
    // 用户登录
    pub fn login(user: web::Json<Value>) -> ResData<DATA<User>> {
        let user = User {
            id: -1,
            name: user.get("name").unwrap().to_string(),
            password: user.get("password").unwrap().to_string(),
        };
        match database::select_user_by_name(&user.name) {
            Ok(result) => {
                if result.password == user.password {
                    // 登录成功
                    ResData::ok(DATA::DATA(result))
                } else {
                    ResData::err(DATA::MSG("密码错误".to_string()))
                }
            }
            Err(_err) => {
                // 没查询到用户，调用新增用户方法
                match database::insert_user(&user) {
                    Ok(_result) => {
                        // 注册成功，成功登录
                        ResData::ok(DATA::DATA(
                            database::select_user_by_name(&user.name).unwrap(),
                        ))
                    }
                    Err(err) => ResData::err(DATA::MSG(format!("登录出错啦：{:#?}", err))),
                }
            }
        }
    }
}
