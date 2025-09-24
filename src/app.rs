#[derive(Debug)]
pub struct App;

impl App {
    pub fn app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn app_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    pub async fn run() {}
}
