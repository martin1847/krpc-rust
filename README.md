
# krpc - rust 实现

主要用来实现一些

* `CPU`密集的工作
* 图像处理等适合`native`的
* 二进制处理

通过`gRPC`的多语言能力，选择最合适的语言。


## 构建

```bash
KRPC_APP_NAME=youApp cargo build
```

## 用法

```rust
//src/demo/hello.rs
//rpcurl $REMOTE/$KRPC_APP_NAME/Demo/hello  -d '"Martin你好"'
// 1. 注册一元函数
krpc::reg_my_fn!();

// 2. 实现
impl UnaryFn for My {
    async fn on_req(&self, request: UnaryRequest) -> UnaryResponse {
        let json_quoted_string = request.into_inner().json;
        let input = crate::util::remove_quotes(&json_quoted_string);
        out_json(format!("\"Hello 你好 :  {}, this is Rust !\"", input))
    }
}
```

##  运行


```rust
// src/main.rs
// 发布你的mod/fns
// (default:env)KRPC_BIND=0.0.0.0:50051
krpc::serve_rpc_mods!(image{captcha}, demo{hello});

```


```bash
KRPC_APP_NAME=youApp cargo run
```
