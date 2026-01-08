use crate::cache::CacheManager;

#[derive(Default)]
pub struct Scheduled {

}

impl Scheduled {
    pub async fn enqueue_jobs(&self,now: chrono::DateTime<chrono::Local>,sorted_set: &Vec<String>){
        let mut n = 0;
        let cache = CacheManager::instance();
    }
}