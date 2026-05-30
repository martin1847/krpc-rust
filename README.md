
# KRPC - rust 实现

https://github.com/martin1847/krpc

主要用来实现一些:

* `CPU`密集的工作
* 图像处理等适合`native`的
* 二进制处理

## 构建

```bash
KRPC_APP_NAME=MyApp cargo build
```

## 用法

```rust
//examples/demo/hello.rs
// 1. 注册一元函数
krpc::reg_my_fn!();

// 2. 实现，可借由 `krpc::inline_me!();` 展开。
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
// examples/demo-server.rs
// 发布你的mod/fns
// (default:env)KRPC_BIND=0.0.0.0:50051
krpc::serve_rpc_mods!(image{captcha}, demo{hello});

```


```bash
KRPC_APP_NAME=demo-server cargo run --example demo-server --features svr
# export REMOTE=http://127.0.0.1:50051
rpcurl $REMOTE/$KRPC_APP_NAME/Demo/hello  -d '"Martin你好"'
```

## 集成测试

先在一个终端启动 demo server：

```bash
KRPC_APP_NAME=demo-server KRPC_BIND=0.0.0.0:50051 cargo run --example demo-server --features svr
```

再在另一个终端运行集成测试：

```bash
KRPC_APP_NAME=test-server cargo test --test integration_test --all-features
```
