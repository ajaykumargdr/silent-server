pub use silent_server::service_server::{Service, ServiceServer};
use std::sync::{mpsc::Sender, Mutex};
use std::time::Duration;
use tonic::{Request, Response};

static AWAKE_CHANNEL: Mutex<Option<Sender<()>>> = Mutex::new(None);

mod silent_server {
    tonic::include_proto!("server");
}

#[derive(Debug, Default)]
pub struct RpcService;

#[tonic::async_trait]
impl Service for RpcService {
    async fn start_server(&self, _request: Request<()>) -> Result<Response<()>, tonic::Status> {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        *AWAKE_CHANNEL.lock().unwrap() = Some(tx);

        std::thread::spawn(move || {
            let child = super::start().unwrap();

            loop {
                if matches!(rx.recv_timeout(Duration::from_millis(500)), Err(_)) {
                    super::stop(child).unwrap();
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

#[test]
#[should_panic]
fn test_mpsc() {
    use std::{thread::sleep, time::Duration};

    // tokio also has mpsc channel
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        sleep(super::Duration::from_millis(1000));
        tx.send(()).unwrap();
    });

    rx.recv_timeout(Duration::from_millis(500)).unwrap();
    dbg!("done");
}
