use tonic::Request;
use tonic::transport::{Endpoint}; 
use protobuf::proto;

pub mod route_guide_gen {
    grpc::include_proto!("", "routeguide");
}

use route_guide_gen::{
    route_guide_client::RouteGuideClient,
    Point,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Create endpoint to connect to
    let endpoint = Endpoint::new("http://[::1]:10000")?; 
    let channel = endpoint.connect().await?;             

    // Create a new client
    let mut client = RouteGuideClient::new(channel); 

    println!("*** SIMPLE RPC ***");
    let point = proto!(Point{
        latitude: 409_146_138,
        longitude: -746_188_906
    });
    let response = client
        .get_feature(Request::new(point))
        .await?.into_inner();

    println!("Response = Name = \"{}\", Latitude = {}, Longitude = {}",
        response.name(),
        response.location().latitude(),
        response.location().longitude());
    Ok(())
}
