use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use yew::prelude::*;
use yew_template::template_html;

#[function_component(App)]
fn app() -> Html {
    let tasks_per_person = get_tasks(3, 16);

    let tasks_html = tasks_per_person.iter().map(|tasks| {
        let ht = tasks.1.iter().map(|task| html! {
            <p>{format!("{}: {}", task.0, task.1)}</p>
        }).collect::<Html>();
        html! {
            <>
                <h2>{format!("{}", tasks.0)}</h2>
                { ht }
            </>
        }
    }).collect::<Html>();


    html! {
        <>
            <h1>{ "DBG6 Cleaning!"}</h1>
            <div>
                { tasks_html }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
