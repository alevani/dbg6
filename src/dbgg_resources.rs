use crate::{
    data::{get_groupped_task, subtasks_per_groupped_tasks, Area, Group},
    TaskData, TaskSection,
};
use chrono::{Datelike, Duration, Local, Weekday};
use gloo_console::info;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::{hash_map::Entry, HashMap};

pub fn get_tasks(number_of_subset: usize, subset_sum_target: i32) -> Vec<TaskData> {
    let (division_of_labor, seed) = generate_subsets(number_of_subset, subset_sum_target);
    let mut groupped_task: HashMap<i32, Vec<(Area, &'static str, Group)>> = get_groupped_task();
    let mut subtasks_per_groupped_tasks: HashMap<Group, Vec<(Area, &'static str, Group)>> =
        subtasks_per_groupped_tasks();

    let mut rng: Pcg64 = Seeder::from(seed).make_rng();

    let mut tasks: Vec<HashMap<Area, Vec<String>>> = Vec::new();
    for subset in division_of_labor {
        let mut current_subset_hmapping: HashMap<Area, Vec<String>> = HashMap::new();

        for diff in subset {
            let task_list = groupped_task.get_mut(&diff).unwrap();
            let task = task_list.remove(rng.gen_range(0..task_list.len()));

            for element in if task.2 != Group::Other {
                subtasks_per_groupped_tasks.remove(&task.2).unwrap()
            } else {
                vec![task]
            } {
                //todo maybe I will have to do that randomly too
                //todo WE SHOULD!!! otherwise if A as a 1, it will always
                //todo be this one
                match current_subset_hmapping.entry(element.0) {
                    Entry::Vacant(e) => {
                        e.insert(vec![element.1.to_string()]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(element.1.to_string());
                    }
                };
            }
        }
        tasks.push(current_subset_hmapping);
    }

    // todo how to handle when we remove a participant for the week?
    let participants = vec!["G. Alexander", "V. Alexandre", "Henriette", "Jon"];

    tasks
        .iter()
        .enumerate()
        .map(|(i, person)| {
            let sections = person
                .iter()
                .map(|(k, v)| TaskSection {
                    name: k.to_string(),
                    tasks: v.to_owned(),
                })
                .collect();

            TaskData {
                holder: participants[i].to_string(),
                task_section: sections,
            }
        })
        .collect()
}

pub fn generate_subsets(number_of_subset: usize, subset_sum_target: i32) -> (Vec<Vec<i32>>, String) {
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
    let mut date = Local::now().date_naive();

    // Will display the same list for Monday as well
    // As some of us like to clean there.
    if date.weekday() == Weekday::Mon {
        date = Local::now().date_naive() - Duration::days(1);
    }

    let date_string = date.format("%Y/%m/%U").to_string();
    let mut rng_try = 0;

    // Shlag solution:
    // since the algorithm is sub-optimal, everytime a solution does not
    // work, we increment the seed, until it does ;).
    // Suboptimal, but ok as we work with small arrays
    loop {
        rng_try += 33;
        
        let rng_seed = format!("{date_string}{rng_try}");
        let mut rng: Pcg64 = Seeder::from(rng_seed.clone()).make_rng();
        
        // - Setup  Data holders
        let mut v: Vec<Vec<i32>> = vec![vec![]; number_of_subset];

        // v = current subset's sum
        let mut v_d: Vec<i32> = vec![0; number_of_subset];

        let mut tasks = vec![9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1];
        let mut tasks_size = tasks.len();

        'inner: loop {
            let value = tasks.remove(rng.gen_range(0..tasks_size));
            tasks_size -= 1;
            let mut index = 0_usize;

            loop {
                if v_d[index] + value <= subset_sum_target {
                    v_d[index] += value;
                    v[index].push(value);
                    break;
                }

                index += 1;

                if index > number_of_subset - 1 {
                    break 'inner;
                }
            }

            if tasks_size == 0 {
                return (v, rng_seed);
            }
        }
    }
}
