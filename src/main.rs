use dbgg_resources::get_tasks_for_member;
use std::net::SocketAddr;

pub mod dbgg_resources;


fn main() {
    get_tasks_for_member();    
}

async fn ping() -> &'static str {
    "I am alive!"
}
