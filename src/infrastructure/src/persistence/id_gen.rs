use std::sync::Arc;

use once_cell::sync::Lazy;

use commonx::snowflake_id::SnowflakeIdGenerator;

use crate::config::APP_CONFIG;

static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    let config = &APP_CONFIG.snow_generator;
    Arc::new(SnowflakeIdGenerator::new(config.machine_id, config.node_id))
});

pub fn next_id() -> i64 {
    let id_gen = ID_GENERATOR.clone();
    id_gen.next_id()
}
