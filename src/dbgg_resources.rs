use chrono::Local;
use core::num;
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
    // let mut used_task_per_group: HashMap<i32, i32> = HashMap::new();
    // let division_of_labor = get_tasks_for_member();

    // for subset in division_of_labor {
    //     let mut s = Vec::new();
    //     for difficulty in subset {
    //         let index = used_task_per_group.get(&difficulty).unwrap_or(&0).to_owned();
    //         let task = GROUPPED_TASK.get(&difficulty).unwrap()[index as usize];
    //         used_task_per_group.insert(difficulty, index + 1);

    //         if task.2 != Group::Other {
    //             s.push(SUBTASKS_PER_GROUPPED_TASKS.get(&task.2).unwrap());

    //         } else {
    //             let t = vec![task];
    //             s.push(&t);

    //         }
    //     }
    // }
}

pub fn get_tasks_for_member(number_of_subset: usize) -> Vec<Vec<i32>> {
    /*
    Assumption for this simple algorithm to always work:
    - The total sum must be A (48 in our case)
    - One should be able to divide A by 2, 3, and 4
    - Each subset's sum (a.k.a target) must be (Number of participant / 48)
    - GCD = 48 / Max number of participant
    - The highest number must not exceed target

    Here is an example
    Number of participant = 4
    Target = 12
    A = [9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1] = 48


    subsets 1: [9, 2, 1]
    subsets 2: [9, 1, 1, 1]
    subsets 3: [8, 2, 2]
    subsets 3: [3, 5, 1, 1, 1, 1]
    */

    // - Setup RNG
    let date = Local::now().date_naive();
    // todo Handle that Monday should be considered Sunday
    let date_string = date.format("%Y/%m/%U").to_string();
    let mut rng_try = 0;
    
    // Shlag solution:
    // since the algorithm is sub-optimal, everytime a solution does not
    // work, we increment the seed, until it does ;).
    // Suboptimal, but ok as we work with small arrays
    loop {
        rng_try += 1;
        let mut rng: Pcg64 = Seeder::from(format!("{date_string}{rng_try}")).make_rng();
        
        // - Setup  Data holders
        let mut v: Vec<Vec<i32>> = vec![vec![]; number_of_subset];

        // v = current subset's sum
        let mut v_d: Vec<i32> = vec![0; number_of_subset];

        let mut tasks = vec![9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1];
        let mut tasks_size = tasks.len();

        'inner: loop {
            let value = tasks.remove(rng.gen_range(0..tasks_size));
            tasks_size -= 1;
            let mut index = 0 as usize;

            loop {
                // todo replace 12 by target
                if v_d[index] + value <= 12 {
                    v_d[index] = v_d[index] + value;
                    v[index as usize].push(value);
                    break;
                }

                index += 1; // will never exceed (Max number of participant - 1)
                            // As per the assumptions
                if index > 3 {
                    break 'inner;
                }
            }

            if tasks_size == 0 {
                return v;
            }
        }
    }

    // data.members.into_iter().for_each(|member| {

    //     //todo does not need to be a new random gen for each, one per week is enough

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
