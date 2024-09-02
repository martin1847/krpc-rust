#![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

// use tonic_clt as tonic;
use super::proto::{InputProto, OutputProto};
use tonic::codegen::http::Uri;
use tonic::codegen::*;

#[derive(Debug, Clone)]
pub struct KrpcClient<T> {
    inner: tonic::client::Grpc<T>,
    // codec: tonic::codec::ProstCodec<InputProto,OutputProto>
}

impl KrpcClient<tonic::transport::Channel> {
    /// Attempt to create a new client by connecting to a given endpoint.
    // pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    // where
    //     D: TryInto<tonic::transport::Endpoint>,
    //     D::Error: Into<StdError>,
    // {
    //     let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
    //     Ok(Self::new(conn))
    // }

    pub async fn connect<T>(url: T) -> Result<Self, tonic::transport::Error> where T: std::convert::AsRef<[u8]> + 'static{
        let uri =
            http::uri::Uri::from_maybe_shared(url).expect("please input currort http(s) url!");
        //.as_ref().to_string()
        // println!("found uri {}",uri);
        KrpcClient::connect_uri(uri).await
    }

    pub async fn connect_uri(uri: http::uri::Uri) -> Result<Self, tonic::transport::Error>
// where
    //     D: TryInto<tonic::transport::Endpoint>,
    //     D::Error: Into<StdError>,
    {
        let ep = tonic::transport::Endpoint::from(uri);
        let ep = if ep.uri().scheme() == Some(&http::uri::Scheme::HTTPS) {
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .expect("Failed to install rustls crypto provider");
            let tls = tonic::transport::ClientTlsConfig::new()
                .with_native_roots()
                // .with_webpki_roots()
                .assume_http2(true);
            // .domain_name("idemo.xxxx.com");
            ep.tls_config(tls)?
        } else {
            ep
        };

        let channel = ep.connect().await?;
        Ok(Self::new(channel))
    }
}

impl<T> KrpcClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn new(inner: T) -> Self {
        let inner = tonic::client::Grpc::new(inner);
        Self { inner }
    }
    pub fn with_origin(inner: T, origin: Uri) -> Self {
        let inner = tonic::client::Grpc::with_origin(inner, origin);
        // let codec:tonic::codec::ProstCodec<InputProto,OutputProto> = tonic::codec::ProstCodec::default();
        Self { inner }
    }
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> KrpcClient<InterceptedService<T, F>>
    where
        F: tonic::service::Interceptor,
        T::ResponseBody: Default,
        T: tonic::codegen::Service<
            http::Request<tonic::body::BoxBody>,
            Response = http::Response<
                <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
            >,
        >,
        <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
            Into<StdError> + Send + Sync,
    {
        KrpcClient::new(InterceptedService::new(inner, interceptor))
    }
    /// Compress requests with the given encoding.
    ///
    /// This requires the server to support it otherwise it might respond with an
    /// error.
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.inner = self.inner.send_compressed(encoding);
        self
    }
    /// Enable decompressing responses.
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.inner = self.inner.accept_compressed(encoding);
        self
    }
    /// Limits the maximum size of a decoded message.
    ///
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_decoding_message_size(limit);
        self
    }
    /// Limits the maximum size of an encoded message.
    ///
    /// Default: `usize::MAX`
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_encoding_message_size(limit);
        self
    }
    pub async fn call(
        &mut self,
        full_path: &str,
        req: impl tonic::IntoRequest<InputProto>
    ) -> std::result::Result<tonic::Response<OutputProto>, tonic::Status> {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        // let req = tonic::Request::new(InputProto { json: json_data.into() });
        // request: impl tonic::IntoRequest<InputProto>
        let codec: tonic::codec::ProstCodec<InputProto, OutputProto> =
            tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::try_from(full_path).unwrap();
        // let mut req = request.into_request();
        // req.extensions_mut().insert(GrpcMethod::new("", ""));

        self.inner.unary(req.into_request(), path, codec).await
    }
}

pub fn req_from(json: &str) -> tonic::Request<InputProto> {
    tonic::Request::new(InputProto { json: json.to_owned() })
}