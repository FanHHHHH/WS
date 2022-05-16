use serde::{Deserialize, Serialize};

/**
 * view model 层
 */

#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub imageurl: Option<String>,
    pub profile: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}
