use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

fn main() {
    // get_tasks_for_member();
    println!("{:?}", get_tasks(4, 12));
}
