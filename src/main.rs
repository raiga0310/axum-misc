use std::sync::atomic::{AtomicUsize, Ordering};
use axum::{
  extract::State,
  routing::get,
  Router,
  response::IntoResponse,
};
use std::net::SocketAddr;

pub trait OperationData: Clone + Send + Sync + 'static {
  fn get(&self) -> usize;
  fn increment(&self);
  fn decrement(&self);
}

struct SharedData {
  counter: AtomicUsize
}

impl SharedData {
  fn new() -> Self {
    Self {
      counter: AtomicUsize::new(0),
    }
  }
}

impl Clone for SharedData {
    fn clone(&self) -> Self {
        Self {
        counter: AtomicUsize::new(self.counter.load(Ordering::Relaxed)),
        }
    }
}

impl OperationData for SharedData {
  fn get(&self) -> usize {
    self.counter.load(Ordering::Relaxed)
  }
  fn increment(&self) {
    self.counter.fetch_add(1, Ordering::Relaxed);
  }
  fn decrement(&self) {
    self.counter.fetch_sub(1, Ordering::Relaxed);
  }
}

#[tokio::main]
async fn main() {
  let data = SharedData::new();
  let app = create_app(data);
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

fn create_app<D: OperationData>(data: D) -> Router {
  Router::new()
    .route("/data", get(data_handler::<D>))
    .with_state(data)
}

async fn data_handler<D: OperationData>(State(data): State<D>) -> impl IntoResponse {
  let current = data.get();
  data.increment();
  format!("Current: {}", current)
}
