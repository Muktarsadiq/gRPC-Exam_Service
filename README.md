# ExamService - gRPC Exam Result Management System

A high-performance, async gRPC service built in Rust for managing and retrieving exam results with both unary and server-streaming RPC capabilities.

## Overview

ExamService demonstrates a production-ready implementation of gRPC communication patterns using the Tonic framework. The service provides exam result queries with support for both single requests and real-time streaming responses.

## Architecture

### Components

**Server (`server.rs`)**

- `ExamServiceImpl`: Core service implementation with thread-safe exam data management
- Uses `Arc<RwLock<HashMap>>` for concurrent read access to exam records
- Implements two RPC methods: unary and server-streaming

**Client (`client.rs`)**

- `ExamServiceClient`: gRPC client for communicating with the server
- Demonstrates both unary and streaming request patterns
- Async/await based response handling with Tokio

**Protocol (`exam.proto`)**

- Service definition with two RPC methods
- Message schemas for requests and responses
- Proto3 syntax for compatibility

## Features

✅ **Unary RPC** - Single request/response pattern for direct exam result queries
✅ **Server-Streaming RPC** - Server sends multiple streamed responses for long-running operations
✅ **Thread-Safe Data** - Arc<RwLock> ensures safe concurrent access
✅ **Async/Await** - Built on Tokio for non-blocking operations
✅ **Error Handling** - Proper gRPC status codes and error propagation
✅ **Structured Logging** - Request/response visibility for debugging

## Getting Started

### Prerequisites

- Rust 1.70+
- Cargo
- protoc (Protocol Buffers compiler)

### Installation

1. Clone the repository and navigate to the project directory:

```bash
cd exam-service
```

2. Add required dependencies to `Cargo.toml`:

```toml
[dependencies]
tonic = "0.10"
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
futures = "0.3"
prost = "0.12"

[build-dependencies]
tonic-build = "0.10"
```

3. Create a `build.rs` for proto compilation:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/exam.proto")?;
    Ok(())
}
```

### Running the Service

**Start the server:**

```bash
cargo run --bin server
```

Expected output:

```
🚀 ExamService listening on [::1]:50051
```

**Run the client (in another terminal):**

```bash
cargo run --bin client
```

Expected output:

```
Unary Response: GetExamResultResponse { student_name: "John Doe", subject: "Math 101", marks_obtained: 95, total_marks: 100, grade: "A+" }
Stream Response: Streamed - Grade: Processing result for 456_physics101 (1/3)
Stream Response: Streamed - Grade: Still working on 456_physics101 (2/3)
Stream Response: Streamed - Grade: Completed result for 456_physics101 (3/3)
```

## API Documentation

### Service Methods

#### GetExamResult (Unary RPC)

Returns exam results for a specific student and exam.

**Request:**

```protobuf
message GetExamResultRequest {
  string student_id = 1;
  string exam_id = 2;
}
```

**Response:**

```protobuf
message GetExamResultResponse {
  string student_name = 1;
  string subject = 2;
  int32 marks_obtained = 3;
  int32 total_marks = 4;
  string grade = 5;
}
```

#### GetExamResultStream (Server-Streaming RPC)

Streams exam result processing updates in real-time.

**Request:** Same as `GetExamResultRequest`
**Response:** Multiple `GetExamResultResponse` messages streamed sequentially

## Pre-populated Data

The service comes with sample exam data:

| Student ID | Exam ID | Student Name | Subject     | Marks  | Grade |
| ---------- | ------- | ------------ | ----------- | ------ | ----- |
| 123        | math101 | John Doe     | Math 101    | 95/100 | A+    |
| 456        | phy101  | Jane Smith   | Physics 101 | 88/100 | A     |

Query format: `{student_id}_{exam_id}`

## Project Structure

```
exam-service/
├── src/
│   ├── server.rs           # gRPC server implementation
│   └── client.rs           # gRPC client implementation
├── proto/
│   └── exam.proto          # Protocol buffer definitions
├── Cargo.toml              # Project dependencies
└── README.md               # This file
```

## Key Technologies

- **Tonic** - Async gRPC framework for Rust
- **Tokio** - Async runtime for concurrent operations
- **Protocol Buffers** - Efficient serialization format
- **tokio-stream** - Stream utilities for Tokio

## Learning Objectives

This project demonstrates:

- gRPC service design patterns (unary and server-streaming)
- Rust async programming with Tokio
- Protocol Buffer usage and code generation
- Thread-safe data structures with Arc<RwLock>
- Error handling in distributed systems

## Extending the Service

### Add Unary Client Streaming

Implement a method where the client sends multiple requests and receives a single aggregated response.

### Add Bidirectional Streaming

Enable simultaneous request and response streaming for interactive applications.

### Database Integration

Replace in-memory HashMap with a persistent database (PostgreSQL, MongoDB, etc.).

### Authentication

Add TLS/mTLS support and token-based authorization.

### Metrics & Observability

Integrate tracing and metrics collection for production monitoring.

## Performance Considerations

- **Connection Reuse**: gRPC uses HTTP/2 multiplexing for efficient connection handling
- **Concurrency**: Tokio's work-stealing scheduler handles thousands of concurrent requests
- **Memory**: Arc<RwLock> minimizes locking overhead for read-heavy workloads
- **Serialization**: Protocol Buffers provide compact, efficient message encoding

## Troubleshooting

**Error: "failed to resolve: use of undeclared type `ExamServer`"**

- Ensure you're importing `ExamServiceServer as ExamServer` from the generated module

**Error: "unresolved import `futures`"**

- Add `futures = "0.3"` to your `Cargo.toml` dependencies

**Connection refused on client startup**

- Verify the server is running on `[::1]:50051`
- Check firewall settings for IPv6 loopback access

## License

MIT

A practical deep-dive into gRPC communication patterns.
