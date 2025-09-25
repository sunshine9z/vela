use std::sync::{Arc, Mutex};

use configx::APP_CONFIG;
use once_cell::sync::Lazy;

use commonx::snowflake_id::SnowflakeIdGenerator;

static ID_GENERATOR: Lazy<Arc<Mutex<SnowflakeIdGenerator>>> = Lazy::new(|| {
    let config = &APP_CONFIG.snowgenera;
    Arc::new(Mutex::new(SnowflakeIdGenerator::new(
        config.machine_id,
        config.node_id,
    )))
});

pub fn gen_id() -> i64 {
    let id_gen = ID_GENERATOR.lock().unwrap();
    id_gen.real_time_generate()
}
