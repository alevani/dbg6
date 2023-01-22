use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use yew::prelude::*;
use yew_template::template_html;

#[function_component(App)]
fn app() -> Html {
    let tasks_per_person = get_tasks(4, 12);

    let tasks_html = tasks_per_person.iter().map(|tasks| {
        let ht = tasks.iter().map(|task| html! {
            <p>{format!("{}: {}", task.0, task.1)}</p>
        }).collect::<Html>();
        html! {
            <>
                <p>{" -- -- -- "}</p>
                { ht }
            </>
        }
    }).collect::<Html>();


    html! {
        <>
            <h1>{ "DBG6 Cleaning!"}</h1>
            <div>
                <h3>{"Task for {todo name}"}</h3>
                { tasks_html }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
