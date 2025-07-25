# Getting Started with gRPC-Rust

Get hands-on with gRPC for Rust in this interactive codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->

Perfect for Rust developers new to gRPC, those seeking a refresher, or anyone
building distributed systems. No prior gRPC experience needed!

## How to use this directory

- [start_here](start_here/) directory serves as a starting point for the
codelab.
- [completed](completed/) directory showcases the finished code, giving you a
peak of how the final implementation should look like.

## Before you begin

### What you’ll learn

* Get hands-on with gRPC for Rust in this interactive codelab\! Perfect for Rust
  developers new to gRPC, those seeking a refresher, or anyone building
  distributed systems. No prior gRPC experience needed\!
* Build a complete gRPC service from scratch, learning:
  * Protocol Buffers (protobuf): Define service contracts & data.
  * gRPC Code Generation: Auto-generate Rust code.
  * Client/Server Communication: Implement seamless interactions.
  * Testing & Debugging: Ensure reliability & correctness.
* You'll gain:
  * A working gRPC service in Rust.
  * Hands-on experience with Protocol Buffers and code generation.
  * Skills to design, build, & test gRPC clients and servers.
  * A strong foundation in gRPC for real-world projects.

### What you’ll need

* A computer with internet connection

### What you'll build

Our example is a simple route mapping application that lets clients get
information about features on their route, create a summary of their route, and
exchange route information such as traffic updates with the server and other
clients.

With gRPC we can define our service once in a `.proto` file and generate clients
and servers in any of gRPC’s supported languages, which in turn can be run in
environments ranging from servers inside a large data center to your own tablet
— all the complexity of communication between different languages and
environments is handled for you by gRPC. We also get all the advantages of
working with protocol buffers, including efficient serialization, a simple IDL,
and easy interface updating.

### Prerequisites

* [**Tonic**](https://github.com/hyperium/tonic.git), the open source repository that gRPC-Rust is build off on
```sh
$ git clone https://github.com/hyperium/tonic.git
// git clone https://github.com/arjan-bal/tonic.git
```
* [**Rust**](https://www.rust-lang.org/).
    * Follow installation instructions [here](https://www.rust-lang.org/tools/install).
* [**Bazel 8.31**](https://bazel.build/).
    * Follow installation instructions [here](https://github.com/bazelbuild/bazel/releases).
* [**Protocol buffer**](https://developers.google.com/protocol-buffers) **compiler**, `protoc`, [version 3](https://protobuf.dev/programming-guides/proto3).
    * For installation instructions, see [Protocol Buffer Compiler Installation](https://grpc.io/docs/protoc-installation/).
    * NOTE: Must need a version of Protoc 3.27.1 or higher.
* **Rust plugins** for the protocol compiler:
```sh
$ cd tonic/protoc-gen-rust-grpc
$ bazel build //src:protoc-gen-rust-grpc
$ PLUGIN_PATH="$(pwd)/bazel-bin/src/protoc-gen-rust-grpc"
```

* Update your PATH so that the protoc compiler can find the plugins:

```sh
export PATH="$(pwd)/bazel-bin/src/:$PATH"
```

* Use [this as a starting point](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2Fgrpc-ecosystem%2Fgrpc-codelabs%2Ftree%2Fmain%2Fcodelabs%2Fgrpc-go-getting-started%2Fstart_here) for this codelab.

## Defining protobuf messages and services

Our first step is to define the gRPC *service* and the method *request* and
*response* types using [protocol buffers](https://protobuf.dev/overview).

Let’s start by defining the messages and service in [this file](start_here/routeguide/route_guide.proto).

### Define the proto Messages

Our `.proto` file contains protocol buffer message type definitions for all the
request and response types used in our service methods.

Let’s define the `Point` message type.  `Point` are represented as
latitude-longitude pairs in the E7 representation.  For the purpose of this
codelabs, we will be using `integer` to define latitude and longitude.

```proto
message Point {
  int32 latitude = 1;
  int32 longitude = 2;
}
```

Let’s also define the `Feature` message type. A `Feature` names something at a
given point using a `string` field.

```proto
message Feature {
  // The name of the feature.
  string name = 1;

  // The point where the feature is detected.
  Point location = 2;
}
```

### Define the RouteGuide service

To define a service, you specify a named service in your `.proto` file:

```proto
service RouteGuide {
  // Definition of the service goes here
}
```

### Define the RPC method in the service

Then you define `rpc` methods inside your service definition, specifying their
request and response types.  In this section of the codelab, let’s define
`GetFeature` method that returns the named `Feature` for the given `Point.`

This would be Unary RPC method \- A *simple RPC* where the client sends a
request to the server using the stub and waits for a response to come back, just
like a normal function call.

```proto
// Obtains the feature at a given position.
rpc GetFeature(Point) returns (Feature) {}
```

> [!TIP]
>  For the complete .proto file, see [routeguide/route_guide.proto](/grpc-go-getting-started/completed/routeguide/route_guide.proto).

## Generating client and server code

Next we need to generate the gRPC client and server interfaces from our `.proto`
service definition. 

### Dependencies
Edit `Cargo.toml` and add all the dependencies we'll need for this example:

```toml
[package]
edition = "2021"
license = "MIT"
name = "getting-started"

[[bin]]
name = "routeguide-server"
path = "src/server/server.rs"

[[bin]]
name = "routeguide-client"
path = "src/client/client.rs"

[features]
routeguide = ["dep:async-stream", "dep:tokio-stream", "dep:rand", "dep:serde", "dep:serde_json"]
full = ["routeguide"]
default = ["full"]

[dependencies]
# Common dependencies
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
prost = "0.14"
tonic = { git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen" }
tonic-protobuf = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-protobuf" }
grpc = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "grpc"}

# Optional dependencies
async-stream = { version = "0.3", optional = true }
tokio-stream = { version = "0.1", optional = true }
tokio-util = { version = "0.7.8", optional = true }
tower = { version = "0.5", optional = true }
rand = { version = "0.9", optional = true }
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
prost-types = { version = "0.14", optional = true }
http = { version = "1", optional = true }
hyper = { version = "1", optional = true }
hyper-util = { version = "0.1.4", optional = true }
tokio-rustls = { version = "0.26.1", optional = true, features = ["ring", "tls12"], default-features = false }
hyper-rustls = { version = "0.27.0", features = ["http2", "ring", "tls12"], optional = true, default-features = false }
tower-http = { version = "0.6", optional = true }
protobuf = { version = "4.31.1-release"}

[build-dependencies]
tonic-protobuf-build = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-protobuf-build" }
tonic-build = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-build" }
```

### Compiling and Building Proto  
Create a `build.rs` file at the root of your crate. A build.rs script is a Rust program that Cargo executes before compiling your main project. Its purpose is to perform tasks like generating source code, linking to non-Rust libraries, or setting environment variables that influence the build process.

In this case, we will be putting the command to compile and build the `.proto` file in build.rs. We will use gRPC's tonic_protobuf_build crate to generate code from the `.proto` file.
```rust
fn main() {
    let proto = "src/routeguide/routeguide.proto";
    tonic_build::compile_protos(proto).unwrap();
    tonic_protobuf_build::CodeGen::new()
        .include("src/routeguide")
        .inputs(["routeguide.proto"])
        .generate_and_compile()
        .unwrap();
}
```
Now, run 
```shell
$ cargo build
```

That's it. The generated code contains:

- Struct definitions for message types `Point` and `Feature`.
- A service trait we'll need to implement: `route_guide_server::RouteGuide`.
- A client type we'll use to call the server: `route_guide_client::RouteGuideClient<T>`.

If your are curious as to where the generated files are, keep reading. The mystery will be revealed
soon! We can now move on to the fun part.

## Creating the server

First let’s look at how we create a `RouteGuide` server. There are two parts to
making our `RouteGuide` service do its job:
* Implementing the service trait generated from our service definition.
* Running a gRPC server to listen for requests from clients.

> [!TIP]
>  For the complete server implementation, see [server.go](completed/server/server.go)

Let’s implement RouteGuide in `server/server.rs`. `server.rs` has code that is commented out in order to generate code from the proto file. Please uncomment the code starting at this step.

### Implementing RouteGuide

We can start by defining a struct to represent our service, we can do this on `main.rs` for now:

```rust
#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}
```

Next, we need to implement the `route_guide_server::RouteGuide` trait that is generated in our build step.
The generated code is placed inside our target directory, in a location defined by the `OUT_DIR`
environment variable that is set by cargo. For our example, this means you can find the generated
code in a path similar to `target/debug/build/routeguide/out/routeguide.rs`.

You can learn more about `build.rs` and the `OUT_DIR` environment variable in the [cargo book].

We can use Tonic's `include_proto` macro to bring the generated code into scope:

```rust
pub mod routeguide {
    tonic::include_proto!("routeguide");
}

use routeguide::route_guide_server::{RouteGuide, RouteGuideServer};
use routeguide::{Feature, Point};
```

**Note**: The token passed to the `include_proto` macro (in our case "routeguide") is the name of
the package declared in our `.proto` file, not a filename, e.g "routeguide.rs".

With this in place, we can stub out our service implementation:

#### Unary RPC

The `routeGuideServer` implements all our service methods. Let’s look at
`get_feature` which just gets a `Point` from the client and returns the
corresponding feature information from its database in a `Feature`.

```rust
#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        println!("GetFeature = {:?}", request);
        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(request.get_ref()) {
                return Ok(Response::new(feature.clone()));
            }
        }
        Ok(Response::new(Feature::default()))
    }
}
```

The method is passed the client’s `Point`
protocol buffer request. It returns a `Feature` protocol buffer object with the
response information. In the method we populate the `Feature`
with the appropriate information, and then `return` it.

## Starting the server

Once we’ve implemented all our methods, we also need to start up a gRPC server
so that clients can actually use our service. The following snippet shows how we
do this for our `RouteGuide` service:

The features that RouteGuideService will instianted with will be from `route_guide_db.json` and will need a helper function from `data.rs`. Uncomment all the code in `data.rs`. Then, fill in main().

```rust
mod data;
use tonic::transport::Server;
```
```rust
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
```

To build and start a server, we:

1. Specify the port we want to use to listen for client requests 
2. Create a service with features loaded in
3. Create an instance of the gRPC server using `RouteGuideServer::new()` using the service we created.
4. Register our service implementation with the gRPC server.
5. Call `serve()` on the server with our port details to do a blocking wait
   until the process is killed or `Stop()` is called.

## Creating the client

In this section, we’ll look at creating a Rust client for our RouteGuide service. You can see our complete example client code in examples/src/routeguide/client.rs.

Like in the server case, we'll start by bringing the generated code into scope:

```rust
use tonic::Request;
use tonic::transport::{Channel, Endpoint}; 

pub mod route_guide_gen {
    grpc::include_proto!("", "routeguide");
}

use route_guide_gen::{
    route_guide_client::RouteGuideClient,
    Point,
};
```

> [!TIP]
>  For the complete server implementation, see [client.go](completed/client/client.go)

### Creating a stub

To call service methods, we first need to create a gRPC *channel* to communicate
with the server. We create this by first creating an endpoint, connecting to that endpoint, and passing the channel made when connected to
`RouteGuideClient::new` as follows:

> [!NOTE]
>  serverAddr can be configured by passing in `addr` flag. Defaults to `localhost:50051`

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Create endpoint to connect to
    let endpoint = Endpoint::new("http://[::1]:10000")?; 
    let channel = endpoint.connect().await?;             

    // Create a new client
    let mut client = RouteGuideClient::new(channel); 
}
```

### Calling service methods

Now let’s look at how we call our service methods. Note that in gRPC-Rust, RPCs
operate in a blocking/synchronous mode, which means that the RPC call waits for
the server to respond, and will either return a response or an error.

#### Simple RPC

Calling the simple RPC `GetFeature` is nearly as straightforward as calling a local method.

```rust
println!("*** SIMPLE RPC ***");
let point = proto!(Point{
    latitude: 409_146_138,
    longitude: -746_188_906
});
let response = client
    .get_feature(Request::new(point))
    .await?.into_inner();
Ok(())
```

As you can see, we call the method on the stub we got earlier. In our method
parameters we create and populate a request protocol buffer object (in our case
`Point`). If the call
doesn’t return an error, then we can read the response information from the
server from the first return value.

```rust
println!("Response = Name = \"{}\", Latitude = {}, Longitude = {}",
    response.name(),
    response.location().latitude(),
    response.location().longitude());
```

## Try it out

First, to run our Client and Server, let's add them as binary targets to our crate. We need to edit our Cargo.toml accordingly:

```toml
[[bin]]
name = "routeguide-server"
path = "src/server/server.rs"

[[bin]]
name = "routeguide-client"
path = "src/client/client.rs"
```

Then, excute the following commands from the working directory:

1. Run the server:

```sh
cd server
cargo run --bin routeguide-server 
```

2. Run the client from another terminal:

```sh
cd client
cargo run --bin routeguide-client
```

You’ll see output like this:

```
*** SIMPLE RPC ***

FEATURE: Name = "Berkshire Valley Management Area Trail, Jefferson, NJ, USA", Lat = 409146138, Lon = -746188906
```
> [!NOTE]
> We’ve omitted timestamps from the client and server trace output shown in this page

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Work through the [Basics tutorial](https://grpc.io/docs/languages/go/basics/).
* Explore the [API reference](https://grpc.io/docs/languages/go/api).