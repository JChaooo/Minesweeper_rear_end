// struct 转 json 的crate
use serde::Serialize;
// 自定义响应状态
#[derive(Debug, Serialize)]
pub enum STATE {
    OK,
    ERR,
}
// 定义响应数据的枚举
#[derive(Debug, Serialize)]
pub enum DATA<T> {
    MSG(String),
    DATA(T)
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
    // 如果数据处理成功，调用OK，返回OK以及数据
    pub fn ok(data: T) -> Self {
        ResData {
            state: STATE::OK,
            data,
        }
    }
    // 如果数据处理出错，调用ERR，
    pub fn err(msg: T) -> Self {
        ResData {
            state: STATE::ERR,
            data: msg,
        }
    }
}
