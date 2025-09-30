use vela::app::App;

#[tokio::main]
async fn main() {
    let _guards = infrastructurex::logger::init().unwrap();

    infrastructurex::web_info!("web_info test");
    App::run().await;
}
