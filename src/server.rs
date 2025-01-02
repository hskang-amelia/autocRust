use tonic::{transport::Server, Request, Response, Status};
use vehicle_status::vehicle_service_server::{VehicleService, VehicleServiceServer};
use vehicle_status::{VehicleRequest, VehicleStatus, VehicleStreamRequest};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use chrono::Local;

// proto에서 생성된 코드를 가져옵니다
pub mod vehicle_status {
    tonic::include_proto!("vehicle_status");
}

// 서비스 구현을 위한 구조체
#[derive(Default)]
pub struct VehicleServiceImpl {}

// VehicleService 트레이트 구현
#[tonic::async_trait]
impl VehicleService for VehicleServiceImpl {
    // 단일 응답 메서드
    async fn get_vehicle_status(
        &self,
        request: Request<VehicleRequest>,
    ) -> Result<Response<VehicleStatus>, Status> {
        let vehicle_id = request.into_inner().id;
        
        let response = VehicleStatus {
            timestamp: Local::now().to_rfc3339(),
            id: vehicle_id,
            status: "정상".to_string(),
            speed: 60.0,
            latitude: 37.5665,
            longitude: 126.9780,
            extra_data: vec![0, 1, 2, 3],
        };

        Ok(Response::new(response))
    }

    // 스트리밍 응답 메서드
    type StreamVehicleStatusStream = ReceiverStream<Result<VehicleStatus, Status>>;

    async fn stream_vehicle_status(
        &self,
        request: Request<VehicleStreamRequest>,
    ) -> Result<Response<Self::StreamVehicleStatusStream>, Status> {
        let count = request.into_inner().count;
        let (tx, rx) = mpsc::channel(128);

        // 백그라운드 태스크로 데이터 스트리밍
        tokio::spawn(async move {
            for i in 0..count {
                let status = VehicleStatus {
                    timestamp: Local::now().to_rfc3339(),
                    id: i,
                    status: "스트리밍".to_string(),
                    speed: 70.0,
                    latitude: 37.5665,
                    longitude: 126.9780,
                    extra_data: vec![0, 1, 2, 3],
                };

                if tx.send(Ok(status)).await.is_err() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = VehicleServiceImpl::default();

    println!("서버 시작: {}", addr);

    Server::builder()
        .add_service(VehicleServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}