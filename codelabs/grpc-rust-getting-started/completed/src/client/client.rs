use tonic::Request;

use routeguide::route_guide_client::RouteGuideClient;
use routeguide::{Point};

pub mod routeguide {
    tonic::include_proto!("routeguide/routeguide");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RouteGuideClient::connect("http://[::1]:10000").await?;

    println!("*** SIMPLE RPC ***");
    let response = client
        .get_feature(Request::new(Point {
            latitude: 409_146_138,
            longitude: -746_188_906,
        }))
        .await?;
    println!("RESPONSE = {response:?}");

    Ok(())
}

