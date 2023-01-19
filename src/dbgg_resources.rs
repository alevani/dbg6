use chrono::Local;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::default;

use crate::data::{self, GROUPPED_TASK, SUBTASKS_PER_GROUPPED_TASKS};


#[derive(Debug)]
pub enum Area {
    Kitchen,
    Bathroom,
    LivingRoom,
    Entrance,
    Outdoor,
    Everywhere,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Group {
    Bathroom,
    Trashs,
    WipeKitchen,
    Vacuum,
    Outdoor,
    Other,
    Default,
}

pub fn a() {
    //vec!["Vanini", "Gamerdinger", "Henriette", "Jon"]
    /*
        Since the DP subset like algorithm we use for the
        division of labor is only dealing with an array of number
        and has no notion of the link between the numbers and the
        task it came from, here is how we will go about thing.

        Each week, we calculate the current seed for the RND based
        on YYYY/MM/WW (if monday, day - 1).
        We then shuffle the list of number, and calculate the subsets.

        The one thing that never gets shuffled is the HashMap of tasks
        T = HashMap<difficulty: i32, Vec<tasks>>.

        With the new subsets, we can then map each number to an item
        of the hashmap, just taking the first one in the stack.

        subset1.map(|s| T.remove(s) (<- make sure to remove from stack if occupied))

        Then, we attribute to a random member the subset, still based on the weekly
        RNG.

        This way, we keep the distribution always random.
    */
    let mut used_task_per_group: HashMap<i32, i32> = HashMap::new();
    let division_of_labor = get_tasks_for_member();
    division_of_labor.into_iter().map(|subset| {
        subset.into_iter().map(|difficulty| {
            let index = *used_task_per_group.get(&difficulty).unwrap_or(&0);
            let task = &GROUPPED_TASK.get(&difficulty).unwrap()[index as usize];
            
            used_task_per_group.insert(difficulty, index + 1);

            if task.2 != Group::Other {
                SUBTASKS_PER_GROUPPED_TASKS.get(&task.2).unwrap()
            } else {
                &vec![*task]
            }
        });
    });
}

// unsure yet how to store that
pub fn get_tasks_for_member() -> Vec<Vec<i32>> {
    // While I think about the algorithm, I will assume I receive the correct date
    vec![
        vec![9, 2, 1],
        vec![9, 1, 1, 1],
        vec![8, 2, 2],
        vec![3, 5, 1, 1, 1, 1],
    ]

    // let data = get_colletive_information();

    // println!("{data:?}");

    // // todo what if they want to clean on the monday?
    // let date = Local::now().date_naive();
    // let date_string = date.format("%Y/%m/%U").to_string();

    // let mut used_task: HashSet<i32> = HashSet::new();

    // data.members.into_iter().for_each(|member| {
    //     // Generates a custom seed based on
    //     // Name - Year/Month/Week
    //     // This way, the result will only vary from a week to another,
    //     // for each member.
    //     //todo does not need to be a new random gen for each, one per week is enough
    //     let mut rng: Pcg64 = Seeder::from(format!("{}{date_string}", member.name)).make_rng();
    //     let mut index = rng.gen_range(0..data.number_of_task);
    //     let mut current_tasks_difficulty = 0;
    //     let mut member_current_task: Vec<&Task> = Vec::new();
    //     let mut task: &Task;

    //     //? To begin with, you can probably do a simple backtracking.
    //     //? You add the element, when it doesn't work for someone else, backtrack
    //     //? though that might be a problem because I use a fixed seed for the generator
    //     println!("----{}---", member.name);
    //     while current_tasks_difficulty < data.max_complexity_per_member {
    //         loop {

    //             task = data.tasks.get(index as usize).unwrap();
    //             if !used_task.contains(&index) /*&& (current_tasks_difficulty + task.difficulty) <= data.max_complexity_per_member */{
    //                 break;
    //             }

    //             index = rng.gen_range(0..data.number_of_task);
    //         }
    //         println!("{task:?}");

    //         used_task.insert(index);
    //         let task = data.tasks.get(index as usize).unwrap();
    //         current_tasks_difficulty += task.difficulty;
    //         member_current_task.push(task);
    //     }
    //     println!("{member_current_task:?}");
    //     println!("{current_tasks_difficulty:?}");
    // });
}
