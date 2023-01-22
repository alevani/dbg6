use dbgg_resources::get_tasks_for_member;

pub mod data;
pub mod dbgg_resources;

fn main() {
    // get_tasks_for_member();
    println!("{:?}", get_tasks_for_member(4 as usize));
}