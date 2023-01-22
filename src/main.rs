use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use yew::prelude::*;
use yew_template::template_html;

#[function_component(App)]
fn app() -> Html {
    let tasks = get_tasks(4, 12);
    let videos = tasks[0].iter().map(|task| html! {
        <p>{format!("{}: {}", task.0, task.1)}</p>
    }).collect::<Html>();

    html! {
        <>
            <h1>{ "DBG6 Cleaning!"}</h1>
            <div>
                <h3>{"Task for {todo name}"}</h3>
                { videos }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
