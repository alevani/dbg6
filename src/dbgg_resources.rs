use axum::Json;

use crate::errors::AppError;

pub struct Member {
    pub name: &'static str
}

pub(crate) fn get_members() -> Result<Json<Vec<Member>>, AppError> {
    Ok(Json(vec!["Vanini", "Gamerdinger", "Henriette", "Jon"]
        .iter()
        .map(|name| Member { name: name })
        .collect()))
}

pub enum Area {
    Kitchen,
    Bathroom,
    LivingRoom,
    Entrance,
}

pub struct Task {
    pub area: Area,
    pub name: String,
    pub difficulty: i32,
}