use std::io::{stdin, stdout, Write};

use silent_server::service_client::ServiceClient;
use tonic::Request;

mod silent_server {
    tonic::include_proto!("server");
}

pub struct RpcClient {
    client: ServiceClient<tonic::transport::Channel>,
}

impl RpcClient {
    pub async fn new(host: &str, port: u16) -> Result<RpcClient, Box<dyn std::error::Error>> {
        Ok(Self {
            client: ServiceClient::connect(format!("http://{}:{}", host, port)).await?,
        })
    }

    pub async fn start_server_request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = Request::new(());
        println!("Sending `startServer` request..");

        if let Ok(response) = self.client.start_server(request).await {
            println!("Response message: {:?}", response.get_ref());
        }

        Ok(())
    }

    pub async fn ping_request(&mut self) {
        let request = Request::new(());
        println!("Sending `ping` request..");

        if let Ok(response) = self.client.ping(request).await {
            println!("Response message: {:?}", response.get_ref());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcClient::new("[::1]", 50051).await.unwrap();
    println!("⚡️ Started RPC Client, Waiting for commands 🪖");

    loop{
        print!("=> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input)?;


        match input.trim() {
            "exit" => break,
            "start" => client.start_server_request().await?,
            "ping" => client.ping_request().await,
            _ => println!("Unknown command"),
        }
    }

    Ok(())
}
