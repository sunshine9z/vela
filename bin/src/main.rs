use vela::app::App;

#[tokio::main]
async fn main() {
    let _guards = commonx::logger::init().unwrap();

    commonx::web_info!("web_info test");
    App::run().await;
}
