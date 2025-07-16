# Getting Started with gRPC-Rust (Streaming)

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

* Get hands-on with gRPC for Rust in this interactive codelab\! Perfect for Go
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

* [**Tonic**](https://github.com/hyperium/tonic.git), the open source repository that gRPC-Rust is based off
```sh
$ git clone https://github.com/hyperium/tonic.git
```
* [**Rust**](https://www.rust-lang.org/), any one of the **two latest major** releases of Rust.
    * Follow installation instructions [here](https://www.rust-lang.org/tools/install).
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
* Use [this as a starting point](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2Fgrpc-ecosystem%2Fgrpc-codelabs%2Ftree%2Fmain%2Fcodelabs%2Fgrpc-go-streaming%2Fstart_here) for this codelab.

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

Next a `Rectangle` message which represents a latitude-longitude rectangle,
represented as two diagonally opposite points "lo" and "hi".

```proto
message Rectangle {
  // One corner of the rectangle.
  Point lo = 1;

  // The other corner of the rectangle.
  Point hi = 2;
}
```

Also a `RouteNote` message which represents a message sent while at a given
point.

```proto
message RouteNote {
  // The location from which the message is sent.
  Point location = 1;

  // The message to be sent.
  string message = 2;
}
```

We would also require a `RouteSummary` message.  This message is receieved in
response to a `RecordRoute` rpc which is explained in the next section.  It
contains the number of individual points received, the number of detected
features, and the total distance covered as the cumulative sum of the distance
between each point.

```proto
message RouteSummary {
  // The number of points received.
  int32 point_count = 1;

  // The number of known features passed while traversing the route.
  int32 feature_count = 2;

  // The distance covered in metres.
  int32 distance = 3;

  // The duration of the traversal in seconds.
  int32 elapsed_time = 4;
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
request and response types.  In this section of the codelab, let’s define:

#### ListFeatures

Obtains the Features available within the given Rectangle. Results are streamed
rather than returned at once (e.g. in a response message with a repeated field),
as the rectangle may cover a large area and contain a huge number of features.

An appropriate type for this RPC is *server-side* streaming RPC. A server-side
streaming RPC where the client sends a request to the server and gets a stream
to read a sequence of messages back. The client reads from the returned stream
until there are no more messages. As you can see in our example, you specify a
server-side streaming method by placing the stream keyword before the response
type.

```proto
rpc ListFeatures(Rectangle) returns (stream Feature) {}
```

#### RecordRoute

Accepts a stream of Points on a route being traversed, returning a `RouteSummary`
when traversal is completed.

A *client-side streaming* RPC seems appropriate in this case.  A client-side
streaming RPC where the client writes a sequence of messages and sends them to
the server, again using a provided stream. Once the client has finished writing
the messages, it waits for the server to read them all and return its response.
You specify a client-side streaming method by placing the stream keyword before
the request type. `GetFeature` method that returns the named `Feature` for the
given `Point.`

```proto
rpc RecordRoute(stream Point) returns (RouteSummary) {}
```

#### RouteChat

Accepts a stream of `RouteNotes` sent while a route is being traversed, while
receiving other `RouteNotes` (e.g. from other users).

This is exactly the kind of usecase for *bidirectional streaming*.  A
bidirectional streaming RPC where both sides send a sequence of messages using a
read-write stream. The two streams operate independently, so clients and servers
can read and write in whatever order they like: for example, the server could
wait to receive all the client messages before writing its responses, or it
could alternately read a message then write a message, or some other combination
of reads and writes. The order of messages in each stream is preserved. You
specify this type of method by placing the stream keyword before both the
request and the response.

```proto
rpc RouteChat(stream RouteNote) returns (stream RouteNote) {}
```

> [!TIP]
>  For the complete .proto file, see [routeguide/route_guide.proto](https://github.com/grpc-ecosystem/grpc-codelabs/blob/bdabe90d07065752a66cf449c2513803b35e6cd3/codelabs/grpc-go-streaming/completed/routeguide/route_guide.pb.go).

## Generating client and server code

Next we need to generate the gRPC client and server interfaces from our `.proto`
service definition. 

Tonic can be configured to generate code as part cargo's normal build process. This is very
convenient because once we've set everything up, there is no extra step to keep the generated code
and our `.proto` definitions in sync.

Behind the scenes, Tonic uses Protobuf Rust to handle protocol buffer serialization and code
generation.

Edit `Cargo.toml` and add all the dependencies we'll need for this example:

```toml
[package]
authors = ["Lucio Franco <luciofranco14@gmail.com>"]
edition = "2021"
license = "MIT"
name = "examples"

[[bin]]
name = "routeguide-server"
path = "src/server/server.rs"
required-features = ["routeguide"]

[[bin]]
name = "routeguide-client"
path = "src/client/client.rs"
required-features = ["routeguide"]

[features]
gcp = ["dep:prost-types", "tonic/tls-ring"]
routeguide = ["dep:async-stream", "dep:tokio-stream", "dep:rand", "dep:serde", "dep:serde_json"]
reflection = ["dep:tonic-reflection"]
autoreload = ["dep:tokio-stream", "tokio-stream?/net", "dep:listenfd"]
health = ["dep:tonic-health"]
grpc-web = ["dep:tonic-web", "dep:bytes", "dep:http", "dep:hyper", "dep:hyper-util", "dep:tracing-subscriber", "dep:tower", "dep:tower-http", "tower-http?/cors"]
tracing = ["dep:tracing", "dep:tracing-subscriber"]
uds = ["dep:tokio-stream", "tokio-stream?/net", "dep:tower", "dep:hyper", "dep:hyper-util"]
streaming = ["dep:tokio-stream", "dep:h2"]
mock = ["dep:tokio-stream", "dep:tower", "dep:hyper-util"]
tower = ["dep:tower", "dep:http"]
json-codec = ["dep:serde", "dep:serde_json", "dep:bytes"]
compression = ["tonic/gzip"]
tls = ["tonic/tls-ring"]
tls-rustls = ["dep:http", "dep:hyper", "dep:hyper-util", "dep:hyper-rustls", "dep:tower", "tower-http/util", "tower-http/add-extension", "dep:tokio-rustls"]
tls-client-auth = ["tonic/tls-ring"]
types = ["dep:tonic-types"]
h2c = ["dep:hyper", "dep:tower", "dep:http", "dep:hyper-util"]
cancellation = ["dep:tokio-util"]

full = ["gcp", "routeguide", "reflection", "autoreload", "health", "grpc-web", "tracing", "uds", "streaming", "mock", "tower", "json-codec", "compression", "tls", "tls-rustls", "tls-client-auth", "types", "cancellation", "h2c"]
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
tonic-web = { git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-web", optional = true }
tonic-health = { git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-health", optional = true }
tonic-reflection = { git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-reflection", optional = true }
tonic-types = { git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-types", optional = true }
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
listenfd = { version = "1.0", optional = true }
bytes = { version = "1", optional = true }
h2 = { version = "0.4", optional = true }
tracing = { version = "0.1.16", optional = true }
tracing-subscriber = { version = "0.3", features = ["tracing-log", "fmt"], optional = true }

[build-dependencies]
tonic-protobuf-build = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-protobuf-build" }
tonic-build = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-build" }
```
Create a `build.rs` file at the root of your crate. A build.rs script is a Rust program that Cargo executes before compiling your main project. Its purpose is to perform tasks like generating source code, linking to non-Rust libraries, or setting environment variables that influence the build process.

In this case, we will be putting the command to compile and build the `.proto` file in build.rs. We will use Tonic's tonic_protobuf_build crate to generate code from the `.proto` file.
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

- Struct definitions for message types `Point`, `Feature`, and `Rectangle`.
- A service trait we'll need to implement: `route_guide_server::RouteGuide`.
- A client type we'll use to call the server: `route_guide_client::RouteGuideClient<T>`.

## Creating the server

First let’s look at how we create a `RouteGuide` server. There are two parts to
making our `RouteGuide` service do its job:

* Implementing the service interface generated from our service definition:
  doing the actual “work” of our service.
* Running a gRPC server to listen for requests from clients and dispatch them to
  the right service implementation.

> [!TIP]
>  For the complete server implementation, see [server.go](completed/server/server.go)

Let’s implement RouteGuide in `server/server.go`

### Implementing RouteGuide

We need to implement the generated `RouteGuideService` interface. This is how
the implementation would look

> [!Note]
>  The starter code already has helper function which will load features into the routeGuideServer's features field.

```rust
impl RouteGuide for RouteGuideService {

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        ...
    }

    async fn record_route(
        &self,
        request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        ...
    }

    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;

    async fn route_chat(
        &self,
        request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        ...
    }
}
```

Let us look into the RPC implementation in detail

#### Server-side streaming RPC 

Now let’s look at one of our streaming RPCs. `ListFeatures` is a server-side
streaming RPC, so we need to send back multiple `Feature` s to our client

```rust
async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        println!("ListFeatures = {:?}", request);
        let (tx, rx) = mpsc::channel(4);
        let features = self.features.clone();
        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature.location.as_ref().unwrap(), request.get_ref()) {
                    println!("  => send {feature:?}");
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }
            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
```

As you can see, instead of getting simple request and response objects in our
method parameters, this time we get a request object (the `Rectangle` in which
our client wants to find `Features`) and a special
`RouteGuide_ListFeaturesServer` object to write our responses. In the method, we
populate as many `Feature` objects as we need to return, writing them to the
`RouteGuide_ListFeaturesServer` using its `Send()` method. Finally, as in our
simple RPC, we return a nil error to tell gRPC that we’ve finished writing
responses. Should any error happen in this call, we return a non-nil error; the
gRPC layer will translate it into an appropriate RPC status to be sent on the
wire.

#### Client-side streaming RPC 

Now let’s look at something a little more complicated: the client-side streaming
method `RecordRoute`, where we get a stream of `Points` from the client and
return a single `RouteSummary` with information about their trip. As you can
see, this time the method doesn’t have a request parameter at all. Instead, it
gets a `RouteGuide_RecordRouteServer` stream, which the server can use to both
read and write messages - it can receive client messages using its `Recv()`
method and return its single response using its `SendAndClose()` method.

```rust
async fn record_route(
    &self,
    request: Request<tonic::Streaming<Point>>,
) -> Result<Response<RouteSummary>, Status> {
    println!("RecordRoute");
    let mut stream = request.into_inner();
    let mut summary = RouteSummary::default();
    let mut last_point = None;
    let now = Instant::now();

    while let Some(point) = stream.next().await {
        let point = point?;
        println!("  ==> Point = {point:?}");

        // Increment the point count
        summary.point_count += 1;

        // Find features
        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(&point) {
                summary.feature_count += 1;
            }
        }

        // Calculate the distance
        if let Some(ref last_point) = last_point {
            summary.distance += calc_distance(last_point, &point);
        }

        last_point = Some(point);
    }

    summary.elapsed_time = now.elapsed().as_secs() as i32;

    Ok(Response::new(summary))
}
```

In the method body we use the `RouteGuide_RecordRouteServer`’s `Recv()` method
to repeatedly read in our client’s requests to a request object (in this case a
`Point`) until there are no more messages: the server needs to check the error
returned from `Recv()` after each call. If this is nil, the stream is still good
and it can continue reading; if it’s `io.EOF` the message stream has ended and
the server can return its `RouteSummary`. If it has any other value, we return
the error “as is” so that it’ll be translated to an RPC status by the gRPC
layer.

#### Bidirectional streaming RPC 

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`.

```rust
type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;

async fn route_chat(
    &self,
    request: Request<tonic::Streaming<RouteNote>>,
) -> Result<Response<Self::RouteChatStream>, Status> {
    println!("RouteChat");

    let mut notes = HashMap::new();
    let mut stream = request.into_inner();

    let output = async_stream::try_stream! {
        while let Some(note) = stream.next().await {
            let note = note?;

            let location = note.location.unwrap();

            let location_notes = notes.entry(location).or_insert(vec![]);
            location_notes.push(note);

            for note in location_notes {
                yield note.clone();
            }
        }
    };

    Ok(Response::new(Box::pin(output) as Self::RouteChatStream))
}
```

This time we get a `RouteGuide_RouteChatServer` stream that, as in our
client-side streaming example, can be used to read and write messages. However,
this time we return values via our method’s stream while the client is still
writing messages to their message stream. The syntax for reading and writing
here is very similar to our client-streaming method, except the server uses the
stream’s Send() method rather than `SendAndClose()` because it’s writing
multiple responses. Although each side will always get the other’s messages in
the order they were written, both the client and server can read and write in
any order — the streams operate completely independently.

## Starting the server

Once we’ve implemented all our methods, we also need to start up a gRPC server
so that clients can actually use our service. The following snippet shows how we
do this for our `RouteGuide` service:


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
2. Create an instance of the gRPC server using `RouteGuideServer::new(...)`.
3. Use `data.load()` to load features into `features`
4. Register our service implementation with the gRPC server.
5. Call `serve()` on the server with our port details to do a blocking wait
   until the process is killed or `Stop()` is called.

## Creating the client

In this section, we’ll look at creating a Rust client for our RouteGuide service.


### Creating a stub

To call service methods, we first need to create a gRPC *channel* to communicate
with the server. We create this by passing the server address and port number to
`RouteGuideClient::new(...)` as follows:

> [!NOTE]
>  serverAddr can be configured by passing in `addr` flag. Defaults to `localhost:50051`

```rust
//Create endpoint to connect to
let endpoint = Endpoint::new("http://[::1]:10000")?; 
let channel = endpoint.connect().await?;             
```

Once the gRPC *channel* is set up, we need a client *stub* to perform RPCs by
making Go function calls. We get it using the `RouteGuideClient::new(...)` method
generated from the example `.proto` file.

```rust
// Create a new client
let mut client = RouteGuideClient::new(channel);
```

### Calling service methods

Now let’s look at how we call our service methods. Note that in gRPC-Go, RPCs
operate in a blocking/synchronous mode, which means that the RPC call waits for
the server to respond, and will either return a response or an error.

#### Server-side streaming RPC 

Here’s where we call the server-side streaming method `ListFeatures`, which
returns a stream of geographical `Feature`s.

```rust
async fn print_features(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut point_one = Point::new();
    point_one.set_latitude(400_000_000);
    point_one.set_longitude(-750_000_000);
    let mut point_two = Point::new();
    point_two.set_latitude(400_000_000);
    point_two.set_longitude(-730_000_000);
    let mut rectangle = Rectangle::new();
    rectangle.set_lo(point_one);
    rectangle.set_hi(point_two);
    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        println!("FEATURE: Name = \"{}\", Lat = {}, Lon = {}",
                feature.name(),
                feature.location().latitude(),
                feature.location().longitude());
    }

    Ok(())
}
```

As in the simple RPC, we pass the method a request. However,
instead of getting a response object back, we get back an instance of
`ListFeaturesStream`. The client can use the
`ListFeaturesStream` stream to read the server’s responses. We use
the `ListFeaturesStream`’s `message()` method to repeatedly read in the
server’s responses to a response protocol buffer object (in this case a
`Feature`) until there are no more messages.

#### Client-side streaming RPC 

The client-side streaming method `RecordRoute` is similar to the server-side
method, except that we only pass the method a request and get 
`Response<RouteSummary>` back, which we can use to both *write* and
*read* messages.

```rust
async fn run_record_route(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let point_count: i32 = rng.random_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng))
    }

    println!("Traversing {} points", points.len());
    let request = Request::new(tokio_stream::iter(points));

    match client.record_route(request).await {
        Ok(response) => {
            let response = response.into_inner();
            println!("SUMMARY: Feature Count = {}, Distance = {}", response.feature_count(), response.distance())},
        Err(e) => println!("something went wrong: {e:?}"),
    }

    Ok(())
}
```

#### Bidirectional streaming RPC 

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`. As in the
case of `RecordRoute`, we only pass the method a `Request` object and get back a
`RouteChatStream` that we can use to both write and read messages. However, this time we
return values via our method’s stream while the server is still writing messages
to their message stream.

```rust
async fn run_route_chat(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let mut point = Point::new();
            point.set_latitude( 409146138 + elapsed.as_secs() as i32);
            point.set_longitude(-746188906);
            let mut note = RouteNote::new();
            note.set_location(point);
            note.set_message(format!("at {elapsed:?}"));

            yield note;
        }
    };

    let response = client.route_chat(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("Note: Latitude = {}, Longitude = {}, Message = \"{}\"",
                note.location().latitude(),
                note.location().longitude(),
                note.message());
    }

    Ok(())
}
```

The syntax for reading and writing here is very similar to our client-side
streaming method. Although each side will always get the other’s messages in
the order they were written, both the client and server can read and write in
any order — the streams operate completely independently.

## Try it out

Execute the following commands from the working directory:

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
*** SERVER STREAMING ***

*** CLIENT STREAMING ***
Traversing 58 points
SUMMARY: Feature Count = 0, Distance = 536724194

*** BIDIRECTIONAL STREAMING ***
Note: Latitude = 409146138, Longitude = -746188906, Message = "at 149.76µs"
Note: Latitude = 409146139, Longitude = -746188906, Message = "at 1.00014976s"
Note: Latitude = 409146140, Longitude = -746188906, Message = "at 2.00014976s"
Note: Latitude = 409146141, Longitude = -746188906, Message = "at 3.00014976s"
Note: Latitude = 409146142, Longitude = -746188906, Message = "at 4.00014976s"
Note: Latitude = 409146143, Longitude = -746188906, Message = "at 5.00014976s"
Note: Latitude = 409146144, Longitude = -746188906, Message = "at 6.00014976s"
Note: Latitude = 409146145, Longitude = -746188906, Message = "at 7.00014976s"
Note: Latitude = 409146146, Longitude = -746188906, Message = "at 8.00014976s"
Note: Latitude = 409146147, Longitude = -746188906, Message = "at 9.00014976s"
Note: Latitude = 409146148, Longitude = -746188906, Message = "at 10.00014976s"
Note: Latitude = 409146149, Longitude = -746188906, Message = "at 11.00014976s"
Note: Latitude = 409146150, Longitude = -746188906, Message = "at 12.00014976s"
Note: Latitude = 409146151, Longitude = -746188906, Message = "at 13.00014976s"
Note: Latitude = 409146152, Longitude = -746188906, Message = "at 14.00014976s"
Note: Latitude = 409146153, Longitude = -746188906, Message = "at 15.00014976s"
Note: Latitude = 409146154, Longitude = -746188906, Message = "at 16.00014976s"
```
> [!NOTE]
> We’ve omitted timestamps from the client and server trace output shown in this page

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Work through the [Basics tutorial](https://grpc.io/docs/languages/go/basics/).
* Explore the [API reference](https://grpc.io/docs/languages/go/api).