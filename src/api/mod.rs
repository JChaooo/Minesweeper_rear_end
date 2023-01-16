use crate::{
    database,
    entity::{checkerboard::CheckerBoard, user::User},
};
use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde_json::Value;

// IP,端口
const SERVER: (&str, u16) = ("localhost", 8090);

// 用户登录
// 接收Json对象，注意要使用.service()注册到APP里面去
#[post("/login")]
pub async fn login(user: web::Json<Value>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&User::login(user)).unwrap())
}
// 保存游戏记录
#[post("/save")]
pub async fn save(data: web::Json<Value>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&CheckerBoard::save(data)).unwrap())
}

// 获取游戏记录
#[post("/record")]
pub async fn record(user_id: web::Json<i32>) -> HttpResponse {
    let user_id = user_id.0;
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&database::select_checker_board_by_user_id(user_id)).unwrap())
}

#[post("/delRecord")]
pub async fn del_record(record_id: web::Json<i32>) -> HttpResponse {
    let record_id = record_id.0;
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&database::delete_record(record_id).unwrap()).unwrap())
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(record)
            .service(save)
            .service(del_record)
    })
    .bind(SERVER)?
    .run()
    .await
}
