use chrono::{Datelike, Duration, Local, Weekday};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::{hash_map::Entry, HashMap};

fn main() {
    generate_subsets(4, 12);
}

pub fn generate_subsets(number_of_subset: usize, subset_sum_target: i32) {
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

    // let date_string = date.format("%Y/%m/%U").to_string();
    let date_string = "2023/01/04";

    for date_string in vec![
        // "2023/01/01",
        // "2023/01/02",
        // "2023/01/03",
        "2023/01/04",
        // "2023/02/01",
        // "2023/02/02",
    ] {
        let mut rng_try = 0;
        // Shlag solution:
        // since the algorithm is sub-optimal, everytime a solution does not
        // work, we increment the seed, until it does ;).
        // Suboptimal, but ok as we work with small arrays
        'haha: loop {
            rng_try += 33;
            let mut rng: Pcg64 = Seeder::from(format!("{date_string}{rng_try}")).make_rng();
            println!("{date_string}{rng_try}");
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
                    if v_d[index] + value <= subset_sum_target {
                        v_d[index] = v_d[index] + value;
                        v[index as usize].push(value);
                        break;
                    }

                    index += 1;

                    if index > number_of_subset - 1 {
                        break 'inner;
                    }
                }

                if tasks_size == 0 {
                    println!("{date_string:?}");
                    println!("{v:?}");
                    break 'haha;
                }
            }
        }
    }
}
