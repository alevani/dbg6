use std::collections::{hash_map::Entry, HashMap};
use chrono::Local;
use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;
use std::collections::HashSet;

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

#[derive(Debug)]
pub enum Area {
    Kitchen,
    Bathroom,
    LivingRoom,
    Entrance,
    Outdoor,
    Everywhere,
}

#[derive(Debug, PartialEq, Eq)]
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
    pub difficulty: i32,

    // If you get a task from a group, it makes sense you get all of that same group
    pub group: Group
}

#[derive(Debug)]
pub struct CollectiveInformation {
    pub tasks: Vec<Task>,
    pub members: Vec<Member>,
    pub number_of_task: i32,
    pub num_members: i32,
    pub total_complexity: i32,
    pub max_complexity_per_member: i32,
}

// unsure yet how to store that
pub fn get_tasks_for_member() {
    let data = get_colletive_information();

    println!("{data:?}");
    
    // todo what if they want to clean on the monday?
    let date = Local::now().date_naive();
    let date_string = date.format("%Y/%m/%U").to_string();

    let mut used_task: HashSet<i32> = HashSet::new();

    data.members.into_iter().for_each(|member| {
        // Generates a custom seed based on
        // Name - Year/Month/Week
        // This way, the result will only vary from a week to another,
        // for each member.
        let mut rng: Pcg64 = Seeder::from(format!("{}{date_string}", member.name)).make_rng();
        let mut index = rng.gen_range(0..data.number_of_task);
        let mut current_tasks_difficulty = 0;
        let mut member_current_task: Vec<&Task> = Vec::new();
        let mut task: &Task;
        
        println!("----{}---", member.name);
        while current_tasks_difficulty < data.max_complexity_per_member {
            loop {
                
                task = data.tasks.get(index as usize).unwrap();
                if !used_task.contains(&index) /*&& (current_tasks_difficulty + task.difficulty) <= data.max_complexity_per_member */{
                    break;
                }
                
                index = rng.gen_range(0..data.number_of_task);
            }
            println!("{task:?}");
            
            used_task.insert(index);
            let task = data.tasks.get(index as usize).unwrap();
            current_tasks_difficulty += task.difficulty;
            member_current_task.push(task);
        }
        println!("{member_current_task:?}");
        println!("{current_tasks_difficulty:?}");
    });
}

fn get_colletive_information() -> CollectiveInformation {
    let mut tasks: Vec<Task> = Vec::new();
    let mut number_of_task = 0;
    let mut total_complexity = 0;

    for (area, name, difficulty, group) in vec![
        // (Area::Bathroom, "Clean mirror", 1, Group::Bathroom),
        // (Area::Bathroom, "Clean sink + tap", 1, Group::Bathroom),
        // (Area::Bathroom, "Clean shower (Floor - Shower head)", 2, Group::Bathroom),
        // (Area::Bathroom, "Wipe all surfaces", 1, Group::Bathroom),
        // (Area::Bathroom, "Clean toilet", 4, Group::Bathroom),
        (Area::Bathroom, "Clean Bathroom tasks", 9, Group::Bathroom),
        // (Area::Bathroom, "Empty trash bin", 1, Group::Trashs),
        // (Area::Kitchen, "Empty Trash + Bio Trash + clean bio bin", 4, Group::Trashs),
        // (Area::Kitchen, "Empty Recycling + clean bins", 2, Group::Trashs),
        // (Area::Kitchen, "Clean under sink", 1, Group::Trashs),
        (Area::Kitchen, "Trash related tasks", 8, Group::Trashs),
        // (Area::Bathroom, "Vacuum floor + wash", 2, Group::Vacuum),
        // (Area::Kitchen, "Vacuum floor + wash", 3, Group::Vacuum),
        // (Area::LivingRoom, "Vacuum sofa and chair", 2, Group::Vacuum),
        // (Area::LivingRoom, "Vacuum floor + wash", 2, Group::Vacuum),
        (Area::LivingRoom, "Vacuuming tasks", 9, Group::Vacuum),
        // (Area::Outdoor, "Refund bottles and cans", 3, Group::Outdoor),
        // (Area::Outdoor, "Shopping (have a look + shoppinglist)", 2, Group::Outdoor),
        (Area::Outdoor, "Outdoor tasks", 5, Group::Outdoor),
        // (Area::Kitchen, "Kitchen counter area: Wipe all surfaces + panels", 2, Group::WipeKitchen),
        // (Area::Kitchen, "Table area: Wipe all surfaces + panels", 1, Group::WipeKitchen),
        (Area::Kitchen, "Wipe kitchen tasks", 3, Group::WipeKitchen),
        (Area::Kitchen, "Descale and clean Kettle ", 1, Group::Other),
        (Area::Kitchen, "Clean Toaster", 1, Group::Other),
        (Area::Kitchen, "Clean Oven + Trays ", 2, Group::Other),
        (Area::Kitchen, "Clean Sink", 1, Group::Other),
        (Area::Kitchen, "Clean micro", 1, Group::Other),
        (Area::Kitchen, "Clean Common shelves in fridge", 2, Group::Other),
        (Area::Kitchen, "Clean the 3 vases (cloths and dish brush + onion + pot spoon, palette knife ect)", 1, Group::Other),
        // (Area::Kitchen, "Kitchen counter area: Wipe all surfaces + panels", 2, Group::WipeKitchen),
        // (Area::Kitchen, "Table area: Wipe all surfaces + panels", 1, Group::WipeKitchen),
        (Area::LivingRoom, "Wipe all surfaces Living room (incl. panels)", 1, Group::Other),
        (Area::LivingRoom, "Clean shoe rack, wipe surfaces hall way (incl. panels)", 2, Group::Other),
        (Area::Kitchen, "Wash towels + Cloths (90 degrees)", 1, Group::Other),
        (Area::Everywhere, "Water plants in common areas", 1, Group::Other),
    ] {
        let task = Task { area, name, group, difficulty };
        tasks.push(task);

        number_of_task += 1;
        total_complexity += difficulty;
    }

    let members = get_members();
    let num_members = members.len() as i32;

    println!("{:?}", tasks.iter().map(|a| a.difficulty).collect::<Vec<_>>());

    let arr = [9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1];
    let target = 20;
    let n = 3;

    // todo, for simplicity, the number should be dividable by 2, 3, 4. 
    // todo. For instance, 48 is a perfect candidate: 12, 16, 24

    CollectiveInformation {
        tasks,
        number_of_task,
        total_complexity,
        members,
        num_members,
        max_complexity_per_member: total_complexity / num_members,
    }
}

/*
What algorithm can solve the following problem: 
Generate a function that randomly generate N subset of the following array
[9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1]
each subset's sum needs to be as close as possible to X
Each element of the array should only be used once
Assume know that N divides X without remainder. That means that each subset is a perfect sum to X
The function should return a vector containing the N subset
*/