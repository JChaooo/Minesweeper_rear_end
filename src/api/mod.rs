use std::collections::HashMap;

use crate::{
    entity::{
        checkerboard::CheckerBoard,
        record::{History, Record},
        user::User,
    },
    result::{ResData, STATE},
};
use actix_web::{
    body::MessageBody,
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};

// IP,端口
const SERVER: (&str, u16) = ("127.0.0.1", 8080);

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
pub async fn login(user: Json<User>) -> HttpResponse {
    // TODO 以下三行代码可以尝试使用宏来复用
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(serde_json::to_string(&user).unwrap())
}

// 保存游戏记录
#[get("/record")]
pub async fn record(record: Json<Record>) -> HttpResponse {
    HttpResponse::Ok().content_type("text/plain").body(
        serde_json::to_string(&Record::new(
            User::new("肖记超".to_string(), "JcMeet2000".to_string()),
            History::new(
                "2020/11/21".to_string(),
                CheckerBoard::new(10, 20, HashMap::new()),
            ),
        ))
        .unwrap(),
    )
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
