#![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

// use tonic_clt as tonic;
use tonic::codegen::http::Uri;
use tonic::codegen::*;
use super::proto::{InputProto,OutputProto};

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

    pub async fn connect(url:String) -> Result<Self, tonic::transport::Error>
    {
        let uri = http::uri::Uri::from_maybe_shared(url).expect("please input currort http(s) url!");
        // println!("found uri {}",uri);
        KrpcClient::connect_uri(uri).await
    }

    pub async fn connect_uri(uri: http::uri::Uri) -> Result<Self, tonic::transport::Error>
    // where
    //     D: TryInto<tonic::transport::Endpoint>,
    //     D::Error: Into<StdError>,
    {
        let ep = tonic::transport::Endpoint::from(uri);
        // let scheme = ep.uri().scheme().as_str();
        // if (&uri.scheme()).expect(&format!("please input currort http(s) url {}",uri)) == &http::uri::Scheme::HTTPS {
        // let ep =  if "https" == scheme {
        let ep = if ep.uri().scheme() == Some(&http::uri::Scheme::HTTPS) {
            rustls::crypto::aws_lc_rs::default_provider().install_default().expect("Failed to install rustls crypto provider");
            let tls = tonic::transport::ClientTlsConfig::new()
            .with_native_roots()
            // .with_webpki_roots()
            .assume_http2(true);
             // .domain_name("idemo.wangyuedaojia.com");
            ep.tls_config(tls)?
        }else{
            ep
        };

        let channel  = ep.connect().await?;
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
        Self { inner  }
    }
    pub fn with_origin(inner: T, origin: Uri) -> Self {
        let inner = tonic::client::Grpc::with_origin(inner, origin);
        // let codec:tonic::codec::ProstCodec<InputProto,OutputProto> = tonic::codec::ProstCodec::default();
        Self { inner  }
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
        // json_data: &str
    ) -> std::result::Result<tonic::Response<OutputProto>, tonic::Status> {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        // let req = tonic::Request::new(InputProto { json: json_data.into() });
        // request: impl tonic::IntoRequest<InputProto>
        let codec:tonic::codec::ProstCodec<InputProto,OutputProto> = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::try_from(full_path).unwrap();
        // let mut req = request.into_request();
        // req.extensions_mut().insert(GrpcMethod::new("", ""));
        
        self.inner.unary(req.into_request(), path, codec).await
    }
}

// static 

pub fn req_from(json: &str) -> tonic::Request<InputProto> {
    tonic::Request::new(InputProto { json: json.into() })
}

// pub fn input_string(json: String) -> tonic::Request<InputProto> {
//     tonic::Request::new(InputProto { json })
// }



// pub fn  tls_client(url: &'static str) ->KrpcClient<Client<hyper_rustls::HttpsConnector<HttpConnector>, http_body_util::combinators::UnsyncBoxBody<Bytes, tonic::Status>>> {
  

//     rustls::crypto::aws_lc_rs::default_provider().install_default().expect("Failed to install rustls aws_lc_rs  provider");
          

//     // let mut root_store = rustls::RootCertStore::empty();
//     // for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
//     //     root_store.add(cert).unwrap();
//     // }

//     // let root_store = rustls::RootCertStore {
//     //     roots: webpki_roots::TLS_SERVER_ROOTS.iter().cloned().collect(),
//     // };
//     // // Create a TLS config
//     // let tls_config = rustls::ClientConfig::builder()
//     //     .with_root_certificates(root_store) // .with_native_roots()?
//     //     .with_no_client_auth();

//     // let arc_crypto_provider = std::sync::Arc::new(rustls::crypto::ring::default_provider());
// // let tls_config = rustls_platform_verifier::tls_config_with_provider(arc_crypto_provider).unwrap();
//         let tls_config = rustls_platform_verifier::tls_config();
        
//         println!("tls config {:?}",tls_config);

//         // // Create a TlsConnector from the TLS config
//         // let tls_connector = tokio_rustls::TlsConnector::from(Arc::new(tls_config));

//         // let mut http = HttpConnector::new();
//         // http.enforce_http(false);
//         // hyper_rustls::HttpsConnectorBuilder::new()
//         // .with_tls_config(tls)
//         // .https_or_http()
//         // .enable_http2()
//         // .wrap_connector(http);

//         // // Prepare the HTTPS connector
//         // let https = hyper_rustls::HttpsConnectorBuilder::new()
//         // .with_tls_config(tls_config)
//         // .https_or_http()
//         // .enable_http2()
//         // .build();

//         let mut http = HttpConnector::new();
//     http.enforce_http(false);

//     // We have to do some wrapping here to map the request type from
//     // `https://example.com` -> `https://[::1]:50051` because `rustls`
//     // doesn't accept ip's as `ServerName`.
//     let connector = tower::ServiceBuilder::new()
//         .layer_fn(move |s| {
//             let tls = tls_config.clone();

//             hyper_rustls::HttpsConnectorBuilder::new()
//                 .with_tls_config(tls)
//                 .https_or_http()
//                 .enable_http2()
//                 .wrap_connector(s)
//         })
        
//         // Since our cert is signed with `example.com` but we actually want to connect
//         // to a local server we will override the Uri passed from the `HttpsConnector`
//         // and map it to the correct `Uri` that will connect us directly to the local server.
//         // .map_request(|_| Uri::from_static("https://[::1]:50051"))
//         .service(http);

//     let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);

//         // Create an HTTPS connector with the TlsConnector
//         // let https_connector = HttpsConnector::from((HttpConnector::new(), tls_connector));
    
//         // Create a Hyper client with the HTTPS connector
//         // let client: Client<_,http_body_util::Empty<Bytes>> = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

//         let uri = Uri::from_static(url);
//             // Create the gRPC channel with the TLS configuration
//     // let channel = tonic::transport::Channel::from_shared("https://your_service_domain.com")?
//     // .tls_config(tls_config)?
//     // .connect()?;
//         let mut client = KrpcClient::with_origin(client, uri);
    
//     client
    
// }
