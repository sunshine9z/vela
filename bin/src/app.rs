use tracing::info;

static MODULE_NAME: &str = "[app]";

#[derive(Debug)]
pub struct App;

impl App {
    pub fn app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn app_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    pub async fn run() {
        info!("{MODULE_NAME}: 应用启动");
        info!("{} v{} is running", Self::app_name(), Self::app_version());

        initdb();
    }
}

fn initdb() {
    info!("{MODULE_NAME}: 初始化数据库");
}
