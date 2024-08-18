pub mod proto;
pub mod server;

use proto::{Out, OutputProto};
use tonic::{Response, Status};

#[allow(dead_code)]
pub fn out_error(code: i32, msg: String) -> Result<Response<OutputProto>, Status> {
    Ok(Response::new(OutputProto {
        code: code,
        // data: Some(Data::Utf8(format!("\"Hello {}!\"",input )))
        out: Some(Out::Error(msg)),
    }))
}

#[allow(dead_code)]
pub fn out_json(json_data: String) -> Result<Response<OutputProto>, Status> {
    Ok(Response::new(OutputProto {
        code: 0,
        out: Some(Out::Json(json_data)), // data: Some(Out::Json(format!("\"{}\"",data)))
    }))
}

#[allow(dead_code)]
pub fn out_bytes(data: Vec<u8>) -> Result<Response<OutputProto>, Status> {
    Ok(Response::new(OutputProto {
        code: 0,
        out: Some(Out::Bytes(data)),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
