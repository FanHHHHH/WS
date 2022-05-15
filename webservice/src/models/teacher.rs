use crate::errors::MyError;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// 只用来查询，不作新增
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

// 只用来作新增
#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

// 只用来作更新
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

// // From trait易于出错 建议实现TryFrom Trait
impl TryFrom<web::Json<CreateTeacher>> for CreateTeacher {
    type Error = MyError;

    fn try_from(teacher: web::Json<CreateTeacher>) -> Result<Self, Self::Error> {
        Ok(CreateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone(),
        })
    }
}

// // From trait易于出错 建议实现TryFrom Trait
impl TryFrom<web::Json<UpdateTeacher>> for UpdateTeacher {
    type Error = MyError;

    fn try_from(teacher: web::Json<UpdateTeacher>) -> Result<Self, Self::Error> {
        Ok(UpdateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone(),
        })
    }
}
