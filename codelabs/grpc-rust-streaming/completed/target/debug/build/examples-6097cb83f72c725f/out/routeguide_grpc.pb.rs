/// Generated client implementations.
pub mod route_guide_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
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
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RouteGuideClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RouteGuideClient::new(InterceptedService::new(inner, interceptor))
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
        ///  A server-to-client streaming RPC.
        ///
        ///  Obtains the Features available within the given Rectangle.  Results are
        ///  streamed rather than returned at once (e.g. in a response message with a
        ///  repeated field), as the rectangle may cover a large area and contain a
        ///  huge number of features.
        ///
        pub async fn list_features(
            &mut self,
            request: impl tonic::IntoRequest<super::Rectangle>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Feature>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_protobuf::ProtoCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/routeguide.RouteGuide/ListFeatures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routeguide.RouteGuide", "ListFeatures"));
            self.inner.server_streaming(req, path, codec).await
        }
        ///  A client-to-server streaming RPC.
        ///
        ///  Accepts a stream of Points on a route being traversed, returning a
        ///  RouteSummary when traversal is completed.
        ///
        pub async fn record_route(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::Point>,
        ) -> std::result::Result<tonic::Response<super::RouteSummary>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_protobuf::ProtoCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/routeguide.RouteGuide/RecordRoute",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routeguide.RouteGuide", "RecordRoute"));
            self.inner.client_streaming(req, path, codec).await
        }
        ///  A Bidirectional streaming RPC.
        ///
        ///  Accepts a stream of RouteNotes sent while a route is being traversed,
        ///  while receiving other RouteNotes (e.g. from other users).
        ///
        pub async fn route_chat(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RouteNote>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::RouteNote>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_protobuf::ProtoCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/routeguide.RouteGuide/RouteChat",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routeguide.RouteGuide", "RouteChat"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
