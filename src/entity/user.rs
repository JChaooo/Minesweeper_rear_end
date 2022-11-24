use std::f32::consts::E;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::{database, result::ResData};

use super::record::Record;

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

    pub fn login(user: web::Json<Value>) -> ResData<String> {
        let user = User {
            id: -1,
            name: user.get("name").unwrap().to_string(),
            password: user.get("password").unwrap().to_string(),
        };
        match database::select_user_by_name(&user.name) {
            Ok(result) => {
                if result.password == user.password {
                    // 登录成功
                    ResData::OK("登录成功！".to_string())
                } else {
                    ResData::ERR("密码错误！".to_string())
                }
            }
            Err(err) => {
                // 没查询到用户，调用新增用户方法
                match database::insert_user(&user) {
                    Ok(result) => {
                        // 注册成功，成功登录
                        ResData::OK("注册后登录！".to_string())
                    }
                    Err(err) => ResData::ERR("登录失败:".to_string() + &err.to_string()),
                }
            }
        }
    }
}
