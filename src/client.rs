use tonic::{Request, transport::Channel};
use vehicle_status::vehicle_service_client::VehicleServiceClient;
use vehicle_status::{VehicleRequest, VehicleStreamRequest};

pub mod vehicle_status {
    tonic::include_proto!("vehicle_status");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 클라이언트 연결
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    
    let mut client = VehicleServiceClient::new(channel);

    // 단일 요청 테스트
    println!("단일 요청 테스트:");
    let request = Request::new(VehicleRequest { 
        id: 1,
        request_id: Some("req-1".to_string())
    });
    let response = client.get_vehicle_status(request).await?;
    println!("응답: {:?}", response.into_inner());

    // 스트리밍 요청 테스트
    println!("\n스트리밍 요청 테스트:");
    let stream_request = Request::new(VehicleStreamRequest { 
        count: 5,
        interval_ms: Some(1000)  // 1초 간격
    });
    let mut stream = client
        .stream_vehicle_status(stream_request)
        .await?
        .into_inner();

    // 스트림 데이터 수신
    while let Some(response) = stream.message().await? {
        println!("스트리밍 응답: {:?}", response);
    }

    Ok(())
}