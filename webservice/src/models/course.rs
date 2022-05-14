use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// 只用来查询，不作新增
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: Option<String>,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// 只用来作新增
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: Option<String>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// 只用来作新增
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    // pub teacher_id: i32,
    pub name: Option<String>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}
// 由于是post过来的，需要从web::Json -> CrateCourse
// // From trait易于出错 建议实现TryFrom Trait
// impl From<web::Json<CrateCourse>> for CrateCourse {
//     fn from(course: web::Json<CrateCourse>) -> Self {
//         CrateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),

//             description: course.description,
//             format: course.format,
//             structure: course.structure,
//             duration: course.duration,
//             price: course.price,
//             language: course.language,
//             level: course.level,
//         }
//     }
// }

// // From trait易于出错 建议实现TryFrom Trait
impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),

            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

// // From trait易于出错 建议实现TryFrom Trait
impl TryFrom<web::Json<UpdateCourse>> for UpdateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<UpdateCourse>) -> Result<Self, Self::Error> {
        Ok(UpdateCourse {
            // teacher_id: course.teacher_id,
            name: course.name.clone(),

            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}
