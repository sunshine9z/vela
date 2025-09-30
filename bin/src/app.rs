use infrastructurex::cache::CacheManager;
use infrastructurex::container::load_domains;
use infrastructurex::persistence::init::init_db;
use infrastructurex::web_info;
use interfacesx::init::start_server;

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
        web_info!("{MODULE_NAME}: 应用启动");
        CacheManager::init().await.unwrap();
        web_info!("{MODULE_NAME}: 1. 缓存初始化 ... [成功]");
        init_db().await.unwrap();
        web_info!("{MODULE_NAME}: 2. 数据库初始化 ... [成功]");
        load_domains().await.unwrap();
        web_info!("{MODULE_NAME}: 3. 加载领域模型 ... [成功]");
        web_info!("{MODULE_NAME}: 4. 启动web服务 ...");
        start_server().await.unwrap();
    }
}
