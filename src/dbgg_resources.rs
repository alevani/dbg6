use std::collections::{hash_map::Entry, HashMap};

use axum::{extract::Path, Json};

use crate::errors::AppError;

pub struct Member {
    pub name: &'static str,
    pub id: i32,
}

pub(crate) async fn get_members() -> Result<Json<Vec<Member>>, AppError> {
    Ok(Json(
        vec!["Vanini", "Gamerdinger", "Henriette", "Jon"]
            .iter()
            .enumerate()
            .map(|(id, name)| Member {
                name,
                id: id as i32,
            })
            .collect(),
    ))
}

// pub(crate) async fn task(Path(member): Path<String>) -> Result<Json<Vec<Member>>, AppError> {
//     // compute a hash for the current year-month-week+member.name
//     // then get the N tasks the person will get. it should also take into consideration the complexity
// }

#[derive(Debug)]
pub enum Area {
    Kitchen,
    Bathroom,
    LivingRoom,
    Entrance,
}

#[derive(Debug)]
pub struct Task {
    pub area: Area,
    pub name: &'static str,
}

// unsure yet how to store that
pub(crate) fn get_tasks() -> HashMap<i32, Vec<Task>> {
    let mut tasks: HashMap<i32, Vec<Task>> = HashMap::new();
    
    for (area, name, priority) in vec![
        (Area::Bathroom, "Clean mirror", 1),
        (Area::Bathroom, "Clean sink + tap", 1),
        (Area::Bathroom, "Clean shower (Floor - Shower head)", 3),
    ] {
        let task = Task { area, name };
        tasks.entry(priority).or_default().push(task);
    }

    tasks
}
