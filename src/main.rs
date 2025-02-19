use silent_server::service_server::{Service, ServiceServer};
use std::sync::{mpsc::Sender, Mutex};
use std::{
    io::Write,
    process::{Child, Command, Output, Stdio},
    time::Duration,
};
use tonic::{Request, Response};

static AWAKE_CHANNEL: Mutex<Option<Sender<()>>> = Mutex::new(None);

mod silent_server {
    tonic::include_proto!("server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let app = RpcService::default();

    tonic::transport::Server::builder()
        .add_service(ServiceServer::new(app))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct RpcService;

#[tonic::async_trait]
impl Service for RpcService {
    async fn start_server(&self, _request: Request<()>) -> Result<Response<()>, tonic::Status> {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        *AWAKE_CHANNEL.lock().unwrap() = Some(tx);

        std::thread::spawn(move || {
            let child = start().unwrap();

            loop {
                if matches!(rx.recv_timeout(Duration::from_millis(500)), Err(_)) {
                    stop(child).unwrap();
                    break;
                };
            }
        });

        Ok(Response::new(()))
    }

    async fn ping(&self, _request: Request<()>) -> Result<Response<()>, tonic::Status> {
        let tx = AWAKE_CHANNEL.lock().unwrap().as_ref().unwrap().clone();
        tx.send(()).unwrap();

        Ok(Response::new(()))
    }
}

pub fn start() -> Result<Child, std::io::Error> {
    Command::new("./program").stdin(Stdio::piped()).spawn()
}

pub fn stop(mut child_process: Child) -> Result<Output, std::io::Error> {
    let child_stdin = child_process.stdin.as_mut().unwrap();
    child_stdin.write_all(b"exit")?;
    child_process.wait_with_output()
}

#[test]
#[should_panic]
fn test_mpsc() {
    use std::{thread::sleep, time::Duration};

    // tokio also has mpsc channel
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        sleep(Duration::from_millis(1000));
        tx.send(()).unwrap();
    });

    rx.recv_timeout(Duration::from_millis(500)).unwrap();
    dbg!("done");
}
