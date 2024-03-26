use super::method::Method;
use std::convert::TryFrom;
pub struct Request {
    path: String,
    query_string: Option<String>,//没有请求字符串
    method: Method,              //枚举方法
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String>{
        
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}