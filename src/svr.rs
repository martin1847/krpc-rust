#![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
use super::proto;//, KRPC_APP_NAME};
use std::collections::HashMap;
use tonic::codegen::*;

pub type UnaryRequest = tonic::Request<proto::InputProto>;
pub type UnaryResponse = Result<tonic::Response<proto::OutputProto>, tonic::Status>;

type BoxedFuture = Pin<Box<dyn Future<Output = UnaryResponse> + Send + 'static>>;
pub type ArcUnaryFnPointer = Arc<dyn Fn(UnaryRequest) -> BoxedFuture + Send + Sync>;

/// Generated trait containing gRPC methods that should be implemented for use with UnaryRpcServer.
// #[async_trait]
pub trait UnaryFn: Send + Sync + 'static {
    // fn path(&self) -> &'static str;

    // async fn on_req(
    //     &self,
    //     request: UnaryRequest,
    // ) -> UnaryResponse ;

    fn on_req(&self, request: UnaryRequest) -> impl Future<Output = UnaryResponse> + Send;

    // fn register(&'static self, methods: &mut HashMap<&'static str, ArcUnaryFnPointer>) {
    //     methods.insert(self.path(), Arc::new(|req| Box::pin(self.on_req(req))));
    // }
}

// static  METEHODS :&'static HashMap<&'static str, AsyncUnaryFn> ;

// #[derive(Debug)]
pub struct UnaryRpcServer {
    // inner: Arc<T>,
    fn_map: &'static HashMap<&'static str, ArcUnaryFnPointer>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
}
impl UnaryRpcServer {
    pub fn new(fn_map: &'static HashMap<&'static str, ArcUnaryFnPointer>) -> Self {
        {
            // let inner = Arc::new(inner);
            for (path, f) in fn_map {
                println!("UnaryRpc Registered {:p}【 {path} 】", f);
            }

            Self {
                fn_map,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
    }

    // pub fn with_interceptor<F>(
    //     inner: T,
    //     interceptor: F,
    // ) -> InterceptedService<Self, F>
    // where
    //     F: tonic::service::Interceptor,
    // {
    //     InterceptedService::new(Self::new(inner), interceptor)
    // }

    /// Enable decompressing requests with the given encoding.
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.accept_compression_encodings.enable(encoding);
        self
    }
    /// Compress responses with the given encoding, if the client supports it.
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.send_compression_encodings.enable(encoding);
        self
    }
    /// Limits the maximum size of a decoded message.
    ///
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.max_decoding_message_size = Some(limit);
        self
    }
    /// Limits the maximum size of an encoded message.
    ///
    /// Default: `usize::MAX`
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.max_encoding_message_size = Some(limit);
        self
    }
}

// #[allow(non_camel_case_types)]
struct InnerSvc(ArcUnaryFnPointer);
impl tonic::server::UnaryService<proto::InputProto> for InnerSvc {
    type Response = proto::OutputProto;
    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
    fn call(&mut self, request: tonic::Request<proto::InputProto>) -> Self::Future {
        let async_fn = Arc::clone(&self.0);
        // let inner = rpc.clone()
        let fut = async move { async_fn(request).await };
        Box::pin(fut)
    }
}

impl<B> tonic::codegen::Service<http::Request<B>> for UnaryRpcServer
where
    // T: UnaryRpc,
    B: Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
{
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        // let methods:&'static HashMap<&'static str, AsyncUnaryFn> = global_methods();
        match self.fn_map.get(req.uri().path()) {
            Some(arc_fn) => {
                let accept_compression_encodings = self.accept_compression_encodings;
                let send_compression_encodings = self.send_compression_encodings;
                let max_decoding_message_size = self.max_decoding_message_size;
                let max_encoding_message_size = self.max_encoding_message_size;
                // let inner = rpc_method.clone().as_ref().as_any().downcast_ref::<T>;
                let fut = async move {
                    // let method = ;
                    let codec = tonic::codec::ProstCodec::default();
                    let mut grpc = tonic::server::Grpc::new(codec)
                        .apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        )
                        .apply_max_message_size_config(
                            max_decoding_message_size,
                            max_encoding_message_size,
                        );
                    let res = grpc.unary(InnerSvc(arc_fn.clone()), req).await;
                    Ok(res)
                };
                Box::pin(fut)
            }
            None => Box::pin(async move {
                Ok(http::Response::builder()
                    .status(200)
                    .header("grpc-status", tonic::Code::Unimplemented as i32)
                    .header(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    )
                    .body(empty_body())
                    .unwrap())
            }),
        }

        // match req.uri().path() {
        //     // self.inner.Arc.
        //     "/-external/Captcha/drawJpg"
        //      =>
        //     _ =>
        // }
    }
}
impl Clone for UnaryRpcServer {
    fn clone(&self) -> Self {
        let fn_map = self.fn_map;
        Self {
            fn_map,
            accept_compression_encodings: self.accept_compression_encodings,
            send_compression_encodings: self.send_compression_encodings,
            max_decoding_message_size: self.max_decoding_message_size,
            max_encoding_message_size: self.max_encoding_message_size,
        }
    }
}

// #[const_env::from_env]
// pub const KRPC_APP_NAME : &'static str = "env!KRPC_APP_NAME";

pub const KRPC_APP_NAME : &'static str = env!("KRPC_APP_NAME");

impl tonic::server::NamedService for UnaryRpcServer {
    const NAME: &'static str = &KRPC_APP_NAME;
}

use proto::{Out, OutputProto};
use tonic::{Response, Status};

// #[allow(dead_code)]
pub fn out_error(code: i32, msg: String) -> Result<Response<OutputProto>, Status> {
    Ok(Response::new(OutputProto {
        code: code,
        // data: Some(Data::Utf8(format!("\"Hello {}!\"",input )))
        out: Some(Out::Error(msg)),
    }))
}

// #[allow(dead_code)]
pub fn out_json(json_data: String) -> Result<Response<OutputProto>, Status> {
    Ok(Response::new(OutputProto {
        code: 0,
        out: Some(Out::Json(json_data)), // data: Some(Out::Json(format!("\"{}\"",data)))
    }))
}

// #[allow(dead_code)]
pub fn out_bytes(data: Vec<u8>) -> Result<Response<OutputProto>, Status> {
    Ok(Response::new(OutputProto {
        code: 0,
        out: Some(Out::Bytes(data)),
    }))
}

#[macro_export]
macro_rules! concat_cst_with_mod {
    ($($s: expr),+) => {{

        // // 获取模块名 , 目前固定支持前两级目录
        // src/image/captcha.rs
        //Image
        const SVC_NAME: &str  = const_str::convert_ascii_case!(upper_camel,  const_str::split!(module_path!(), "::")[1]);
        //captcha
        const METHOD_NAME: &str  = const_str::split!(module_path!(), "::")[2];
    

        const STRS: &[&str] = &[
            $($s,)+
            "/",
            SVC_NAME,
            "/",
            METHOD_NAME
        ];

        const TOTAL_LEN: usize = {
            let mut ans = 0;
            let mut arr = STRS;
            while let [x, xs @ ..] = arr {
                ans += x.len();
                arr = xs;
            }
            ans
        };

        const CONST_STR_BUF: [u8; TOTAL_LEN] = {
            let mut buf: [u8; TOTAL_LEN] = [0; TOTAL_LEN];
            let mut cur: usize = 0;
            let mut arr = STRS;
            while let [x, xs @ ..] = arr {
                let bytes = x.as_bytes();
                let mut i = 0;
                while i < bytes.len() {
                    buf[cur] = bytes[i];
                    i += 1;
                    cur += 1;
                }
                arr = xs;
            }
            buf
        };

        unsafe { ::core::str::from_utf8_unchecked(&CONST_STR_BUF) }
    }}
}

#[macro_export]
macro_rules! reg_my_fn {
    () => {
        use krpc::svr::{
            out_bytes, out_error, out_json,
            UnaryFn, UnaryRequest, UnaryResponse
        };

        const API_PATH: &'static str = krpc::concat_cst_with_mod!("/", &krpc::svr::KRPC_APP_NAME);
        pub const FN: My = My(&API_PATH);
        pub struct My(pub &'static str);
    }
}

#[macro_export]
macro_rules! _pub_fns {

    // hello::INSTANCE.register(&mut map);
    // let a = vec![captcha::INSTANCE,hello::INSTANCE];


    // fn register<T: UnaryFn>(map: &mut HashMap<&'static str, ArcUnaryFnPointer>, biz: &'static T) {
    //     map.insert(biz.path(), Arc::new( |r| Box::pin(biz.on_req(r))));
    // }
    // // 注册所有的bizfn(rpc method)

    // 下面写法可以直接UnaryFn声明async on_req.
    // let biz = &captcha::INSTANCE;
    // map.insert(biz.path(), Arc::new(|r| Box::pin(biz.on_req(r))));
    // let biz = &hello::INSTANCE;
    // map.insert(biz.path(), Arc::new(|r| Box::pin(biz.on_req(r))));

    ($($unary_fn: expr),+) => {

        type FnMap = std::collections::HashMap<&'static str, krpc::svr::ArcUnaryFnPointer>;
        const FN_MAP_INIT: std::sync::Once = std::sync::Once::new();
        static mut FN_MAP: Option<FnMap> = None;
        fn get_fn_map() -> &'static FnMap {
            unsafe {
                FN_MAP_INIT.call_once(|| {
                    let mut map:FnMap = std::collections::HashMap::new();

                    use krpc::svr::UnaryFn;
                    $(
                        map.insert($unary_fn.0, std::sync::Arc::new(|r| Box::pin($unary_fn.on_req(r))));
                    )+
                    FN_MAP = Some(map);
                });
                FN_MAP.as_ref().unwrap()
            }
        }
    }
}


#[macro_export]
macro_rules! _start_server {
    () => {
    //     krpc::_start_server!("0.0.0.0:50051");
    // };
    // ($host_port: expr) => {
        let krpc_bind = std::env::var("KRPC_BIND").unwrap_or_else(|_| "0.0.0.0:50051".to_string());
        let addr:core::net::SocketAddr = krpc_bind.parse()?;
        println!("🦀 🟢 KRPC Server【 http://{} 】", addr);
    
        tonic::transport::Server::builder()
            .add_service(krpc::svr::UnaryRpcServer::new(get_fn_map()))
            .serve(addr)
            .await?;
    }
}

#[macro_export]
macro_rules! serve_rpc_mods {

    ($($svc_name:ident { $($fn_name:ident),+ }),+) => {
        
        $(mod $svc_name;)+
        
        krpc::_pub_fns!($($(&$svc_name::$fn_name::FN),+),+);

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {

            krpc::_start_server!();

            Ok(())
        }
    };
}


#[macro_export]
macro_rules! inline_me {
    () => {
        impl UnaryFn for My {
            async fn on_req(&self, request: UnaryRequest) -> UnaryResponse {
                let json = request.into_inner().json;
                out_json(format!("\"TODO !你好， {}, this is Rust KRPC!\"", input))
            }
        }        
    }
}




// #[macro_export]
// macro_rules! current_module_name {
//     () => {{
//         let path = module_path!();
//         let parts: Vec<&str> = path.split("::").collect();
//         *parts.last().unwrap() // 获取模块名,没法转换成const
//     }};
// }