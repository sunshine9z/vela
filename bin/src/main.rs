use loggerx::web_info;
use vela::app::App;

#[tokio::main]
async fn main() {
    let _guards = loggerx::init().unwrap();

    web_info!("web_info test");
    App::run().await;
}
