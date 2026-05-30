use crate::demo::remove_quotes;

krpc::reg_my_fn!();

impl UnaryFn for My {
    async fn on_req(&self, request: UnaryRequest) -> UnaryResponse {
        let json_quoted_string = request.into_inner().json;
        let input = remove_quotes(&json_quoted_string);
        out_json(format!("\"Hello 你好 :  {}, this is Rust !\"", input))
    }
}
