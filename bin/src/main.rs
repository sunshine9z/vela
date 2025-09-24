use bin::app::App;
use tokio::select;

#[tokio::main]
async fn main() {
    let _guards = logger::init().unwrap();
    App::run().await;

    select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("ctrl-c signal received, exit");
        }
    }
}
