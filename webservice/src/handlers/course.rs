use crate::db_access::course;
use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};

use crate::models::course::Course;
use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("发现新课程");
    course::post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> Result<HttpResponse, MyError> {
    course::get_courses_for_teacher_db(&app_state.db, i32::try_from(params.0).unwrap())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    course::get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
    // HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: None,
            time: None,
        });
        let resp = new_course(course, app_state).await.unwrap();
        // println!("resp:{:#?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let params = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
