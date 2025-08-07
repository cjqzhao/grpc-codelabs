use std::sync::Arc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use protobuf::proto;

mod data;
use data::{Feature, Point, RouteGuide, RouteGuideServer};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        println!("GetFeature = {:?}", request);
        let requested_point = request.get_ref();
        for feature in &self.features[..] {
            if feature.location().latitude() == requested_point.latitude() {
                if feature.location().longitude() == requested_point.longitude(){
                    return Ok(Response::new(feature.clone()))
                };
            };    
        }
        Ok(Response::new(Feature::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    println!("RouteGuideServer listening on: {addr}");
    let route_guide = RouteGuideService {
        features: Arc::new(data::load()),
    };
    let svc = RouteGuideServer::new(route_guide);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}


