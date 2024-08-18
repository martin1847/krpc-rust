
# krpc - rust 实现

主要用来实现一些

* `CPU`密集的工作
* 图像处理等适合`native`的
* 二进制处理

通过`gRPC`的多语言能力，选择最合适的语言。


## 用法

```rust
pub const INSTANCE: HelloFn = HelloFn {};
pub struct HelloFn {}
const API_PATH: &'static str = concat_cst!("/", &APP_NAME, "/Demo/hello");
impl UnaryFn for HelloFn {
    fn path(&self) -> &'static str {
        &API_PATH
    }

    async fn on_req(&self, request: UnaryRequest) -> UnaryResponse {
        let input = request.into_inner().json;
        out_json(format!("\"Hello 你好 :  {}, this is Rust !\"", input))
    }
}
```



##  运行

启动服务
```rust
use krpc::{
    init_rpc_methods,
    server::{UnaryFn, UnaryRpcServer},
};
use tonic::transport::Server;

//注册所有的impl UnaryFn
init_rpc_methods!(demo::FN, hello::FN);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;
    let addr = "0.0.0.0:50051".parse()?;
    println!("listen on : {}/{}", addr, krpc::KRPC_APP_NAME);

    Server::builder()
        .add_service(UnaryRpcServer::new(rpc_methods()))
        .serve(addr)
        .await?;

    Ok(())
}

```


```bash
KRPC_APP_NAME=youApp cargo run
```