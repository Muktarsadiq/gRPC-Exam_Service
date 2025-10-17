use std::collections::HashMap;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;

pub mod exam_service {
    tonic::include_proto!("exam");
}

use exam_service::exam_service_server::{ExamService, ExamServiceServer as ExamServer};
use exam_service::{GetExamResultRequest, GetExamResultResponse};

// The core server struct implementing the ExamService gRPC interface.
// Holds a map of exam data keyed by a composite "student_id_exam_id" string.
#[derive(Debug, Clone)]
pub struct ExamServiceImpl {
    // Exam results wrapped in Arc<RwLock<>> for thread-safe access
    exam_data: Arc<RwLock<HashMap<String, GetExamResultResponse>>>,
}

impl ExamServiceImpl {
    // Constructs a new instance of the service with pre-populated exam data.
    pub fn new() -> Self {
        let mut data = HashMap::new();

        data.insert(
            "123_math101".to_string(),
            GetExamResultResponse {
                student_name: "John Doe".to_string(),
                subject: "Math 101".to_string(),
                marks_obtained: 95,
                total_marks: 100,
                grade: "A+".to_string(),
            },
        );

        data.insert(
            "456_phy101".to_string(),
            GetExamResultResponse {
                student_name: "Jane Smith".to_string(),
                subject: "Physics 101".to_string(),
                marks_obtained: 88,
                total_marks: 100,
                grade: "A".to_string(),
            },
        );

        Self {
            exam_data: Arc::new(RwLock::new(data)),
        }
    }
}

#[tonic::async_trait]
impl ExamService for ExamServiceImpl {
    // Handles a unary request to get exam result by student_id and exam_id.
    async fn get_exam_result(
        &self,
        request: Request<GetExamResultRequest>,
    ) -> Result<Response<GetExamResultResponse>, Status> {
        println!("Got a Unary Request: {:?}", request);

        let req = request.into_inner();
        let key = format!("{}_{}", req.student_id, req.exam_id);

        let data = self.exam_data.read().await;

        if let Some(result) = data.get(&key) {
            return Ok(Response::new(result.clone()));
        }

        Err(Status::not_found(format!("No result found for key: {}", key)))
    }

    // Server-Streaming RPC
    type GetExamResultStreamStream = ReceiverStream<Result<GetExamResultResponse, Status>>;

    async fn get_exam_result_stream(
        &self,
        request: Request<GetExamResultRequest>,
    ) -> Result<Response<Self::GetExamResultStreamStream>, Status> {
        println!("Got a Streaming Request: {:?}", request);

        let req = request.into_inner();
        let key = format!("{}_{}", req.student_id, req.exam_id);

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let simulated_results = vec![
                format!("Processing result for {} (1/3)", key),
                format!("Still working on {} (2/3)", key),
                format!("Completed result for {} (3/3)", key),
            ];

            for msg in simulated_results {
                if tx
                    .send(Ok(GetExamResultResponse {
                        student_name: "Streamed".to_string(),
                        subject: "Simulation".to_string(),
                        marks_obtained: 90,
                        total_marks: 100,
                        grade: msg.clone(),
                    }))
                    .await
                    .is_err()
                {
                    println!("Client disconnected before stream finished");
                    break;
                }

                sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let exam_service = ExamServiceImpl::new();

    println!("ExamService listening on {}", addr);

    Server::builder()
        .add_service(ExamServer::new(exam_service))
        .serve(addr)
        .await?;

    Ok(())
}