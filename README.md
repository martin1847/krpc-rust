
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