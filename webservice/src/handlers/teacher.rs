use crate::db_access::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse, ResponseError};

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.try_into()?)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let teachers = get_all_teachers_db(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    get_teacher_details_db(&app_state.db, id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
    // HttpResponse::Ok().json(course)
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    delete_teacher_db(&app_state.db, id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    update_teacher: web::Json<UpdateTeacher>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    update_teacher_details_db(&app_state.db, id, update_teacher.try_into()?)
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
    async fn post_teacher_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let new_teacher = web::Json(CreateTeacher {
            name: "Test Teacher".into(),
            picture_url: Some("www.test_url.com".into()),
            profile: Some("test profile".into()),
        });
        let resp = post_new_teacher(new_teacher, app_state).await.unwrap();
        // println!("resp:{:#?}", resp);
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_teachers_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let resp = get_all_teachers(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_teacher_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let params = web::Path::from(2);
        let resp = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_teacher_fail() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let params = web::Path::from(10001);
        let resp = get_teacher_details(app_state, params).await;

        match resp {
            Ok(_) => println!("出错了"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_teacher_details_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let update_teacher = web::Json(UpdateTeacher {
            name: Some("updatea teacher name test".into()),
            picture_url: Some("updated pic url test".into()),
            profile: Some("update profile test".into()),
        });

        let params = web::Path::from(2);
        let resp = update_teacher_details(app_state, update_teacher, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[ignore]
    #[actix_rt::test]
    async fn delete_teacher_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let params = web::Path::from(1);
        let resp = delete_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn delete_teacher_fail() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not valid.");
        let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let params = web::Path::from(100001);
        let resp = delete_teacher(app_state, params).await;
        match resp {
            Ok(_) => println!("删除出错！"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
