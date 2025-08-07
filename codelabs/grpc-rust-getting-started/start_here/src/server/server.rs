// use std::sync::Arc;
// use tonic::transport::Server;
// use tonic::{Request, Response, Status};
// use protobuf::proto;

// mod data;
// use data::{Feature, Point, RouteGuide, RouteGuideServer};

// #[derive(Debug)]
// pub struct RouteGuideService {
//     features: Arc<Vec<Feature>>,
// }

// #[tonic::async_trait]
// impl RouteGuide for RouteGuideService {
//     async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
//         ///////////////////////////////////////////////////////////////////////////
//         // Codelab Hint: Logic for GetFeature will be added here.
//         //
//         // Steps include:
//         // -   Loop through server's features to find the feature that matches the
//         //     point.
//         // -   Return the feature if found.
//         // -   Return an unnamed feature if no feature is found.
//         ///////////////////////////////////////////////////////////////////////////
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for starting up a gRPC Server will be added here.
	//
	// Steps include:
	//  -   Specify the port we want to use to listen for client requests using
	//  -   Create an instance of the gRPC server using RouteGuideServer::new().
	//  -   Register our service implementation with the server.
	///////////////////////////////////////////////////////////////////////////
}


