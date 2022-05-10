// use super::models::Course;
use sqlx::mysql::MySqlPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: MySqlPool, // 将数据库连接池放在AppState里面，过线程共享使用数据库连接池
}
