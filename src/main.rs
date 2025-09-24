use tokio::select;
use vela::{app::App, banner::BANNER};

#[tokio::main]
async fn main() {
    let _guards = vela::infrastructure::logger::init().unwrap();

    tracing::info!("{BANNER}");
    App::run().await;

    select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("ctrl-c signal received, exit");
        }
    }
}
