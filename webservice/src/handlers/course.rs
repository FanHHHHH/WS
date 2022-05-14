use crate::db_access::course;
use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::models::course::*;
use chrono::Utc;

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    course::post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, MyError> {
    course::get_courses_for_teacher_db(&app_state.db, i32::try_from(params.0).unwrap())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    course::get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
    // HttpResponse::Ok().json(course)
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    course::delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    course::update_course_details_db(
        &app_state.db,
        teacher_id,
        course_id,
        update_course.try_into()?,
    )
    .await
    .map(|id| HttpResponse::Ok().json(id))
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[ignore]
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
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: Some("Test course".into()),
            description: Some("this is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("中文".into()),
            level: Some("Beginner".into()),
        });
        let resp = post_new_course(course, app_state).await.unwrap();
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

        let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
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

    #[actix_rt::test]
    async fn get_one_course_fail() {
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

        let params = web::Path::from((1, 101));
        let resp = get_course_detail(app_state, params).await;

        match resp {
            Ok(_) => println!("出错了"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_details_test() {
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
        let update_course = web::Json(UpdateCourse {
            name: Some("course name changed".into()),
            description: Some("updated course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("中文".into()),
            level: Some("medium".into()),
        });

        let params = web::Path::from((1, 1));
        let resp = update_course_details(app_state, update_course, params)
            .await
            .unwrap();
        // println!("resp:{:#?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
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

        let params = web::Path::from((1, 3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    
    #[actix_rt::test]
    async fn delete_course_fail() {
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

        let params = web::Path::from((1, 101));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("删除出错！"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
