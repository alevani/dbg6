use std::{collections::HashSet, ops::Deref};

use rand::{seq::SliceRandom, Rng};

fn main() {
    let N = 4;
    let X = 12;
    let A = &[9, 8, 9, 5, 3, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1];

    let subsets = subset_sum(A, X, N);
    for subset in subsets {
        println!("{:?}", subset);
    }
}

fn subset_sum(A: &[i32], X: i32, N: usize) -> Vec<Vec<i32>> {
    let mut subsets = vec![];
    let mut used = HashSet::new();
    let subset_size = X / N as i32;
    backtrack(&mut subsets, &mut used, A, 0, subset_size, X);
    subsets
}

fn backtrack(subsets: &mut Vec<Vec<i32>>, used: &mut HashSet<usize>, A: &[i32], start: usize, subset_size: i32, X: i32) {
    println!("{subset_size:?}");
    if subsets.len() == (X / subset_size) as usize {
        return;
    }
    if A[start..].iter().filter(|&&x| !used.contains(&(x as usize))).sum::<i32>() < subset_size {
        return;
    }
    for i in start..A.len() {
        if !used.contains(&i) && A[i] <= subset_size {
            let mut subset = vec![A[i]];
            used.insert(i);
            backtrack(subsets, used, A, i + 1, subset_size - A[i], X);
            used.remove(&i);
            for j in start..A.len() {
                if !used.contains(&j) && A[j] + subset.iter().sum::<i32>() <= subset_size {
                    subset.push(A[j]);
                    used.insert(j);
                }
            }
            if subset.iter().sum::<i32>() == subset_size {
                subsets.push(subset);
            }
            for j in start..A.len() {
                if used.contains(&j) {
                    used.remove(&j);
                }
            }
        }
    }
}