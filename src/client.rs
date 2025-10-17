use tonic::{transport::Channel, Request};
use exam_service::exam_service_client::ExamServiceClient;
use exam_service::GetExamResultRequest;
use futures::StreamExt;

pub mod exam_service {
    tonic::include_proto!("exam");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a gRPC client connection to the server
    let mut client = ExamServiceClient::connect("http://[::1]:50051").await?;

    // Prepare a request for the unary RPC
    let request = Request::new(GetExamResultRequest {
        student_id: "123".to_string(),
        exam_id: "math101".to_string(),
    });

    // Call the unary RPC method
    let response = client.get_exam_result(request).await?;
    println!("Unary Response: {:?}", response.into_inner());

    // Prepare a new request for the streaming RPC
    let request = Request::new(GetExamResultRequest {
        student_id: "456".to_string(),
        exam_id: "physics101".to_string(),
    });

    // Call the streaming RPC method
    let mut stream = client.get_exam_result_stream(request).await?.into_inner();

    // Process the stream as responses arrive asynchronously
    while let Some(response) = stream.next().await {
        match response {
            Ok(reply) => println!(
                "Stream Response: {} - Grade: {}",
                reply.student_name, reply.grade
            ),
            Err(e) => eprintln!("Stream Error: {}", e),
        }
    }

    Ok(())
}