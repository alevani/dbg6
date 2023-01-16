use std::collections::{hash_map::Entry, HashMap};

use axum::{extract::Path, Json};
use chrono::Local;

use crate::errors::AppError;

use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

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
pub enum Group {
    Bathroom,
    Trashs,
    WipeKitchen,
    Vacuum,
    Outdoor,
    Other,
}

#[derive(Debug)]
pub struct Task {
    pub area: Area,
    pub name: &'static str,

    // If you get a task from a group, it makes sense you get all of that same group
    pub group: Group
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

    
    let date = Local::now().date_naive();
    let date_string = date.format("%Y/%m/%U").to_string();

    data.members.into_iter().map(|member| {
        // Generates a custom seed based on
        // Name - Year/Month/Week
        // This way, the result will only vary from a week to another,
        //      for each member.
        let mut rng: Pcg64 = Seeder::from("stripy zebra").make_rng();
    })

    
    println!("{}", rng.gen::<char>());

    Ok(())
}

// unsure yet how to store that
fn get_colletive_information() -> CollectiveInformation {
    let mut tasks: HashMap<i32, Vec<Task>> = HashMap::new();
    let mut number_of_task = 0;
    let mut total_complexity = 0;

    for (area, name, difficulty, group) in vec![
        (Area::Bathroom, "Clean mirror", 1, Group::Bathroom),
        (Area::Bathroom, "Clean sink + tap", 1, Group::Bathroom),
        (Area::Bathroom, "Clean shower (Floor - Shower head)", 3, Group::Bathroom),
        (Area::Bathroom, "Clean toilet", 3, Group::Bathroom),
    ] {
        let task = Task { area, name, group };
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
