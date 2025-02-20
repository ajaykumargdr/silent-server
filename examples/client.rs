use std::time::Duration;
use silent_server::service_client::ServiceClient;
use tonic::Request;

mod silent_server {
    tonic::include_proto!("server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ServiceClient::connect("http://[::1]:50051")
        .await
        .expect("server is not available!");

    let request = Request::new(());
    println!("requesting..");

    if let Ok(response) = client.start_server(request).await {
        println!("Response = {:?}", response);
    }

    std::thread::sleep(Duration::from_secs(5));

    let request = Request::new(());
    if let Ok(response) = client.ping(request).await {
        println!("Response = {:?}", response);
    }

    /*
        loop{
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if input.contains("exit"){
                break;
            }

            let request = Request::new(
                MessageServer{
                    message : input,
                    timestamp: std::time::SystemTime::now().elapsed().unwrap().as_secs()
                }
            );

            let response: Response<MessageClient> = client.chat(request).await?;

            println!("Server Response = {:?}",response);
        }
    */
    Ok(())
}
