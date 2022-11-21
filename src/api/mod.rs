use crate::result::{ResData, STATE};
use actix_web::{
    body::MessageBody,
    get,
    http::{header::ContentType, StatusCode},
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};

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
        .body(ResData::to_json(res_data))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(greet))
        .bind(SERVER)?
        .run()
        .await
}
