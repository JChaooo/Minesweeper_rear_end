// struct 转 json 的crate
use serde::Serialize;
// 自定义响应状态
#[derive(Debug, Serialize)]
pub enum STATE {
    OK,
    ERR,
}
// 自定义响应
#[derive(Serialize)]
pub struct ResData<T> {
    // 响应的状态
    state: STATE,
    // 响应的数据
    data: T,
}

impl<T: Serialize> ResData<T> {
    // 构造函数
    pub fn new(state: STATE, data: T) -> Self {
        ResData { state, data }
    }
    // // ResData转换成json
    // pub fn to_json(self) -> String {
    //     serde_json::to_string(&self).unwrap()
    // }
}
