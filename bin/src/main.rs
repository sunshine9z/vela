use loggerx::web_info;
use tokio::select;
use vela::app::App;

#[tokio::main]
async fn main() {
    let _guards = loggerx::init().unwrap();

    web_info!("web_info test");
    App::run().await;
    select! {
        _ = tokio::signal::ctrl_c() => {
            web_info!("ctrl-c signal received, exit");
        }
    }
}
