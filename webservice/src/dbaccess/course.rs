use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
// use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;

pub async fn get_courses_for_teacher_db(
    pool: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT *
        FROM course 
        WHERE teacher_id = ?"#,
        teacher_id
    )
    .fetch_all(pool) //返回 option类型，可为空
    .await?;

    if let 0 = rows.len() {
        Err(MyError::NotFound("找不到该教师的课程！".to_string()))
    } else {
        Ok(rows)
    }
}

pub async fn get_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT *
    From course
    WHERE teacher_id = ? and id = ?"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;
    match row {
        Some(row) => Ok(row),
        _ => Err(MyError::NotFound("找不到该课程！".to_string())),
    }
}

pub async fn post_new_course_db(
    pool: &MySqlPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let id = sqlx::query!(
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
    VALUES (?, ?, ?, ? ,?, ?, ?, ?, ?)"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level
    )
    .execute(pool)
    .await?
    .last_insert_id();
    get_course_details_db(pool, new_course.teacher_id, id.try_into().unwrap()).await
}

pub async fn delete_course_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
) -> Result<String, MyError> {
    let row_count = sqlx::query!(
        r#"DELETE 
    FROM course 
    WHERE teacher_id = ? and id = ?"#,
        teacher_id,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();
    if let 1 = row_count {
        Ok(format!("删除id:{}的课程成功！", id))
    } else {
        Err(MyError::NotFound("找不到该课程！".into()))
    }
}

pub async fn update_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<String, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        r#"SELECT *
    FROM course
    WHERE teacher_id = ? and id = ?
    "#,
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("找不到课程".into()))?;

    let name = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name.unwrap_or_default()
    };
    let description = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    let language = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };

    let level = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };

    let row_count = sqlx::query!(r#"UPDATE course 
    SET name = ?, description = ?, format = ?, structure = ?, duration = ?, price = ?, language = ?, level = ?
    WHERE teacher_id = ? and id = ? "#, name, description, format, structure, duration, price, language, level, teacher_id, id).execute(pool).await?.rows_affected();
    if let 1 = row_count {
        Ok(format!("更新id:{}的课程成功！", id))
    } else {
        Err(MyError::NotFound("找不到该课程！".into()))
    }
}
