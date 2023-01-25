use futures::future::join_all;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tokio::spawn;
mod bpf_handler;
mod server;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut tasks = Vec::new();
    tasks.push(spawn(bpf_handler::load_and_run_bpf()));
    tasks.push(spawn(server::run_server()));

    join_all(tasks).await;
}


