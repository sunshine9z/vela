pub mod init;
mod job;
mod cron_scheduled;
mod processor;
mod unit_of_work;
mod wokers;
mod worker;
mod scheduled;

static MODULE_NAME: &str = "[processor]";
