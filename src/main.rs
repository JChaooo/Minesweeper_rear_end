mod api;
pub mod database;
pub mod entity;
pub mod result;
fn main() {
    let result = database::init_db();
    println!("数据库初始化结果：{:?}", result);
    let start_server = api::start_server();
    match start_server {
        Ok(_) => {}
        Err(why) => println!("启动失败：{}", why),
    }
}
