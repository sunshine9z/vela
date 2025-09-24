use vela::{app::App, banner::BANNER};

#[tokio::main]
async fn main() {
    println!("{BANNER}");
    App::run().await;
}
