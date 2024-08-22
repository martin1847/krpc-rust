use krpc::clt::{KrpcClient,input_str};



#[tokio::test]
async fn test_clt_hello() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = KrpcClient::connect("http://0.0.0.0:50051").await?;

   
    let method = "/test-server/Demo/str";

    let response = client.call(method,input_str("\"Rust1\"")).await?;

    // let res = response.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}