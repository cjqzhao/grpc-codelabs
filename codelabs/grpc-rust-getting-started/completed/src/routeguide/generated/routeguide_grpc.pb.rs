/// Generated client implementations.
pub mod route_guide_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        // will trigger if compression is disabled
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;

    ///  Interface exported by the server.
    ///
    #[derive(Debug, Clone)]
    pub struct RouteGuideClient<T> {
        inner: tonic::client::Grpc<T>,
    }

    impl<T> RouteGuideClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send  +
        'static, <T::ResponseBody as Body>::Error: Into<StdError> +
        std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }

        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }

        pub fn with_interceptor<F>(inner: T, interceptor: F) ->
        RouteGuideClient<InterceptedService<T, F>> where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<<T as
                tonic::client::GrpcService<tonic::body::Body>>::ResponseBody>
            >,
            <T as
            tonic::codegen::Service<http::Request<tonic::body::Body>>>::Error:
            Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RouteGuideClient::new(InterceptedService::new(inner, interceptor))
        }

        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding)
        -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }

        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding:
        CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }

        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) ->
        Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }

        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) ->
        Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }

        ///  A simple RPC.
        ///
        ///  Obtains the feature at a given position.
        ///
        ///  A feature with an empty name is returned if there's no feature at the given
        ///  position.
        ///
        pub async fn get_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::Point>,
        ) -> std::result::Result<tonic::Response<super::Feature>, tonic::Status> {
           self.inner.ready().await.map_err(|e| {
               tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
           })?;
           let codec = tonic_protobuf::ProtoCodec::default();
           let path = http::uri::PathAndQuery::from_static("/routeguide.RouteGuide/GetFeature");
           let mut req = request.into_request();
           req.extensions_mut().insert(GrpcMethod::new("routeguide.RouteGuide", "GetFeature"));
           self.inner.unary(req, path, codec).await
        }
    }
}
