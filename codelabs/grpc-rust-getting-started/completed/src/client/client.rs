use tonic::Request;
use tonic::transport::{Channel, Endpoint}; 

pub mod route_guide_gen {
    grpc::include_proto!("", "routeguide");
}

use route_guide_gen::{
    route_guide_client::RouteGuideClient,
    Point,
};

#[derive(Debug, Clone)]
pub struct RouteGuideAppClient {
    inner: RouteGuideClient<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Create endpoint to connect to
    let endpoint = Endpoint::new("http://[::1]:10000")?; 
    let channel = endpoint.connect().await?;             

    // Create a new client
    let mut client = RouteGuideClient::new(channel); 

    println!("*** SIMPLE RPC ***");
    let mut point = Point::new();
    point.set_latitude(409_146_138); 
    point.set_longitude(-746_188_906);
    let response = client
        .get_feature(Request::new(point))
        .await?;
    
    println!("RESPONSE = {response:?}");
    Ok(())
}
