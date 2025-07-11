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

* [**Rust**](https://www.rust-lang.org/), any one of the **two latest major** releases of Rust.
    * Follow installation instructions [here](https://www.rust-lang.org/tools/install).
* [**Protocol buffer**](https://developers.google.com/protocol-buffers) **compiler**, `protoc`, [version 3](https://protobuf.dev/programming-guides/proto3).
    * For installation instructions, see [Protocol Buffer Compiler Installation](https://grpc.io/docs/protoc-installation/).
    * NOTE: Must need a version of Protoc 3.27.1 or higher.
* **Go plugins** for the protocol compiler:
    * Install the protocol compiler plugins for Go using the following commands.

```console
# This command will install the plugin that generates code for the messages
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest

# This command installs the plugin that generates code for the services and methods
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

* Update your PATH so that the protoc compiler can find the plugins:

```sh
export PATH="$PATH:$(go env GOPATH)/bin"
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

Tonic can be configured to generate code as part cargo's normal build process. This is very
convenient because once we've set everything up, there is no extra step to keep the generated code
and our `.proto` definitions in sync.

Behind the scenes, Tonic uses Protobuf Rust to handle protocol buffer serialization and code
generation.

Edit `Cargo.toml` and add all the dependencies we'll need for this example:

```toml
[package]
edition = "2021"
license = "MIT"
name = "getting-started"


[features]
routeguide = ["dep:async-stream", "dep:tokio-stream", "dep:rand", "dep:serde", "dep:serde_json"]
full = ["routeguide"]
default = ["full"]

[dependencies]
# Common dependencies
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
prost = "0.14"
tonic = { git = "https://github.com/hyperium/tonic.git", branch = "master" }
# Optional dependencies
tonic-web = { git = "https://github.com/hyperium/tonic.git", branch = "master", package = "tonic-web", optional = true }
tonic-health = { git = "https://github.com/hyperium/tonic.git", branch = "master", package = "tonic-health", optional = true }
tonic-reflection = { git = "https://github.com/hyperium/tonic.git", branch = "master", package = "tonic-reflection", optional = true }
tonic-types = { git = "https://github.com/hyperium/tonic.git", branch = "master", package = "tonic-types", optional = true }
async-stream = { version = "0.3", optional = true }
tokio-stream = { version = "0.1", optional = true }
tokio-util = { version = "0.7.8", optional = true }
tower = { version = "0.5", optional = true }
rand = { version = "0.9", optional = true }
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
tracing = { version = "0.1.16", optional = true }
tracing-subscriber = { version = "0.3", features = ["tracing-log", "fmt"], optional = true }
prost-types = { version = "0.14", optional = true }
http = { version = "1", optional = true }
hyper = { version = "1", optional = true }
hyper-util = { version = "0.1.4", optional = true }
listenfd = { version = "1.0", optional = true }
bytes = { version = "1", optional = true }
h2 = { version = "0.4", optional = true }
tokio-rustls = { version = "0.26.1", optional = true, features = ["ring", "tls12"], default-features = false }
hyper-rustls = { version = "0.27.0", features = ["http2", "ring", "tls12"], optional = true, default-features = false }
tower-http = { version = "0.6", optional = true }
protobuf = { version = "4.31.1-release"}


[build-dependencies]
tonic-protobuf-build = {git = "https://github.com/arjan-bal/tonic.git", branch = "grpc-codegen", package = "tonic-protobuf-build" }
```
Create a `build.rs` file at the root of your crate. A build.rs script is a Rust program that Cargo executes before compiling your main project. Its purpose is to perform tasks like generating source code, linking to non-Rust libraries, or setting environment variables that influence the build process.

In this case, we will be putting the command to compile and build the `.proto` file in build.rs. We will use Tonic's tonic_protobuf_build crate to generate code.
```rust
fn main() {
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
>  The starter code already has helper function which will load features into the routeGuideServer's savedFeatures field.

```go
type routeGuideServer struct {
        ...
}
...

func (s *routeGuideServer) GetFeature(ctx context.Context, point *pb.Point) (*pb.Feature, error) {
        ...
}
```

Let us look into the RPC implementation in detail

#### Unary RPC

The `routeGuideServer` implements all our service methods. Let’s look at
`GetFeature` which just gets a `Point` from the client and returns the
corresponding feature information from its database in a `Feature`.

```go
func (s *routeGuideServer) GetFeature(ctx context.Context, point *pb.Point) (*pb.Feature, error) {
  for _, feature := range s.savedFeatures {
    if proto.Equal(feature.Location, point) {
      return feature, nil
    }
  }
  // No feature was found, return an unnamed feature
  return &pb.Feature{Location: point}, nil
}

```

The method is passed a context object for the RPC and the client’s `Point`
protocol buffer request. It returns a `Feature` protocol buffer object with the
response information and an `error`. In the method we populate the `Feature`
with the appropriate information, and then `return` it along with a nil error to
tell gRPC that we’ve finished dealing with the RPC and that the `Feature` can be
returned to the client.

## Starting the server

Once we’ve implemented all our methods, we also need to start up a gRPC server
so that clients can actually use our service. The following snippet shows how we
do this for our `RouteGuide` service:

> [!NOTE]
>  port can be configured by passing in `port` flag. Defaults to `50051`

```go
lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", *port))
if err != nil {
  log.Fatalf("failed to listen: %v", err)
}
var opts []grpc.ServerOption
grpcServer := grpc.NewServer(opts...)

s := &routeGuideServer{}
s.loadFeatures()
pb.RegisterRouteGuideServer(grpcServer, s)
grpcServer.Serve(lis)
```

To build and start a server, we:

1. Specify the port we want to use to listen for client requests using:
   `lis, err := net.Listen(...)`
2. Create an instance of the gRPC server using `grpc.NewServer(...)`.
3. Use `s.loadFeatures()` to load features into `s.savedFeatures`
4. Register our service implementation with the gRPC server.
5. Call `Serve()` on the server with our port details to do a blocking wait
   until the process is killed or `Stop()` is called.

## Creating the client

In this section, we’ll look at creating a Go client for our RouteGuide service.

> [!TIP]
>  For the complete server implementation, see [client.go](completed/client/client.go)

### Creating a stub

To call service methods, we first need to create a gRPC *channel* to communicate
with the server. We create this by passing the server address and port number to
`grpc.NewClient()` as follows:

> [!NOTE]
>  serverAddr can be configured by passing in `addr` flag. Defaults to `localhost:50051`

```go
conn, err := grpc.NewClient("dns:///"+*serverAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
if err != nil {
	log.Fatalf("fail to dial: %v", err)
}
defer conn.Close()
```

You can use `DialOptions` to set the auth credentials (for example, TLS, GCE
credentials, or JWT credentials) in `grpc.NewClient` when a service requires
them. The `RouteGuide` service doesn’t require any credentials.

Once the gRPC *channel* is set up, we need a client *stub* to perform RPCs by
making Go function calls. We get it using the `NewRouteGuideClient` method
provided by the pb package generated from the example `.proto` file.

```go
client := pb.NewRouteGuideClient(conn)
```

### Calling service methods

Now let’s look at how we call our service methods. Note that in gRPC-Go, RPCs
operate in a blocking/synchronous mode, which means that the RPC call waits for
the server to respond, and will either return a response or an error.

#### Simple RPC

Calling the simple RPC `GetFeature` is nearly as straightforward as calling a local method.

```go
point := &pb.Point{Latitude: 409146138, Longitude: -746188906}
log.Printf("Getting feature for point (%d, %d)", point.Latitude, point.Longitude)
// Call GetFeature method on the client.
feature, err := client.GetFeature(context.TODO(), point)
if err != nil {
  log.Fatalf("client.GetFeature failed: %v", err)
}
```

As you can see, we call the method on the stub we got earlier. In our method
parameters we create and populate a request protocol buffer object (in our case
`Point`). We also pass a `context.Context` object which lets us change our RPC’s
behavior if necessary, such as time-out/cancel an RPC in flight. If the call
doesn’t return an error, then we can read the response information from the
server from the first return value.

```go
log.Println(feature)
```

## Try it out

Execute the following commands from the working directory:

1. Run the server:

```sh
cd server
go run .
```

2. Run the client from another terminal:

```sh
cd client
go run .
```

You’ll see output like this:

```
Getting feature for point (409146138, -746188906)
name:"Berkshire Valley Management Area Trail, Jefferson, NJ, USA" location:<latitude:409146138 longitude:-746188906 >
Getting feature for point (0, 0)
location:<>
```
> [!NOTE]
> We’ve omitted timestamps from the client and server trace output shown in this page

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Work through the [Basics tutorial](https://grpc.io/docs/languages/go/basics/).
* Explore the [API reference](https://grpc.io/docs/languages/go/api).