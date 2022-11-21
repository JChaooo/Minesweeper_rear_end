mod api;
pub mod result;
pub mod entity;
fn main() {
    let start_server = api::start_server();
    match start_server {
        Ok(_) => {}
        Err(why) => println!("启动失败：{}", why),
    }
}
