use std::collections::HashMap;

use crate::{
    database,
    entity::{checkerboard::CheckerBoard, record::Record, user::User},
    result::{ResData, STATE},
};
use actix_web::{
    body::MessageBody,
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, Json},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde_json::Value;

// IP,端口
const SERVER: (&str, u16) = ("192.168.73.22", 8090);

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
#[get("/")]
pub async fn index() -> HttpResponse {
    let res_data = ResData::new(STATE::OK, "Person:{name:肖记超,age:18}");
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&res_data).unwrap())
}

/// TODO 这些请求api的函数体只是为了测试接口，具体实现要写在别处
///
///

// 用户登录
// 接收Json对象，注意要使用.service()注册到APP里面去
#[post("/login")]
pub async fn login(request: HttpRequest, user: web::Json<Value>) -> HttpResponse {
    // TODO 以下三行代码可以尝试使用宏来复用
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&User::login(user)).unwrap())
}

// 保存游戏记录
#[get("/record")]
pub async fn record(record: Json<Record>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string("hhhh").unwrap())
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
            .service(login)
            .service(record)
    })
    .bind(SERVER)?
    .run()
    .await
}
