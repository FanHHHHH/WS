use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::mysql::MySqlPool;

pub async fn get_all_teachers_db(pool: &MySqlPool) -> Result<Vec<Teacher>, MyError> {
    let rows: Vec<Teacher> = sqlx::query_as!(
        Teacher,
        r#"SELECT *
        FROM teacher"#,
    )
    .fetch_all(pool) //返回 option类型，可为空
    .await?;

    if let 0 = rows.len() {
        Err(MyError::NotFound("找不到可用教师！".into()))
    } else {
        Ok(rows)
    }
}

pub async fn get_teacher_details_db(pool: &MySqlPool, id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"SELECT id, name, picture_url, profile
    From teacher
    WHERE id = ?"#,
        id,
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name,
        picture_url: r.picture_url,
        profile: r.profile,
    })
    .map_err(|_err| MyError::NotFound("找不到对应老师！".into()))?;
    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &MySqlPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let id = sqlx::query!(
        r#"INSERT INTO teacher (name, picture_url, profile)
    VALUES (?, ?, ?)"#,
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    )
    .execute(pool)
    .await?
    .last_insert_id();
    get_teacher_details_db(pool, i32::try_from(id).unwrap()).await
}

pub async fn delete_teacher_db(pool: &MySqlPool, id: i32) -> Result<String, MyError> {
    let row_count = sqlx::query!(
        r#"DELETE 
    FROM teacher 
    WHERE id = ?"#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();
    if let 1 = row_count {
        Ok(format!("删除id:{}的教师成功！", id))
    } else {
        Err(MyError::NotFound("找不到该教师！".into()))
    }
}

pub async fn update_teacher_details_db(
    pool: &MySqlPool,
    id: i32,
    update_teacher: UpdateTeacher,
) -> Result<String, MyError> {
    let current_teacher_row = sqlx::query!(
        r#"SELECT id, name, picture_url, profile
    FROM teacher
    WHERE id = ?
    "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("找不到该教师".into()))?;
    let name = if let Some(name) = update_teacher.name {
        name
    } else {
        current_teacher_row.name
    };
    let picture_url = if let Some(picture_url) = update_teacher.picture_url {
        picture_url
    } else {
        current_teacher_row.picture_url.unwrap()
    };
    let profile = if let Some(profile) = update_teacher.profile {
        profile
    } else {
        current_teacher_row.profile.unwrap()
    };

    let row_count = sqlx::query!(
        r#"UPDATE teacher 
    SET name = ?, picture_url = ?, profile = ?
    WHERE id = ?"#,
        name,
        picture_url,
        profile,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();
    if let 1 = row_count {
        Ok(format!("更新id:{}的教师成功！", id))
    } else {
        Err(MyError::NotFound("找不到该教师！".into()))
    }
}
