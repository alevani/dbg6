use std::collections::{hash_map::Entry, HashMap};

use axum::{extract::Path, Json};

use crate::errors::AppError;

#[derive(Debug)]
pub struct Member {
    pub name: &'static str,
    pub id: i32,
}

fn get_members() -> Vec<Member> {
    vec!["Vanini", "Gamerdinger", "Henriette", "Jon"]
        .iter()
        .enumerate()
        .map(|(id, name)| Member {
            name,
            id: id as i32,
        })
        .collect()
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

#[derive(Debug)]
pub struct CollectiveInformation {
    pub tasks: HashMap<i32, Vec<Task>>,
    pub number_of_task: i32,
    pub total_complexity: i32,
    pub members: Vec<Member>,
    pub num_members: i32,
    pub max_complexity_per_member: i32,
}

// unsure yet how to store that
pub(crate) fn get_tasks_for_member() -> Result<(), AppError> {
    let data = get_colletive_information();

    Ok(())
}

// unsure yet how to store that
fn get_colletive_information() -> CollectiveInformation {
    let mut tasks: HashMap<i32, Vec<Task>> = HashMap::new();
    let mut number_of_task = 0;
    let mut total_complexity = 0;

    for (area, name, difficulty) in vec![
        (Area::Bathroom, "Clean mirror", 1),
        (Area::Bathroom, "Clean sink + tap", 1),
        (Area::Bathroom, "Clean shower (Floor - Shower head)", 3),
        (Area::Bathroom, "Clean toilet", 3),
    ] {
        let task = Task { area, name };
        tasks.entry(difficulty).or_default().push(task);

        number_of_task += 1;
        total_complexity += difficulty;
    }

    let members = get_members();
    let num_members = members.len() as i32;
    
    CollectiveInformation {
        tasks,
        number_of_task,
        total_complexity,
        members,
        num_members,
        max_complexity_per_member: total_complexity / num_members,
    }
}
