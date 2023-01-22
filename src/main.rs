use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use yew::prelude::*;

pub struct TaskView {
    pub holder: String,
    pub task_section: Vec<TaskSection>,
}

pub struct TaskSection {
    pub name: String,
    pub tasks: Vec<String>,
}


#[function_component(App)]
fn app() -> Html {
    let task_views = get_tasks(4, 12);

    let tasks_html = task_views.iter().map(|task_view| {
        
        let ht = task_view.task_section.iter().map(|sections| {
            let inner_ht = sections.tasks.iter().map(|t_name| html! {
                <p>{t_name}</p>
            }).collect::<Html>();

            html! {
                <>
                    <h3>{format!("{}", sections.name)}</h3>
                    { inner_ht }
                </>
            }
        }).collect::<Html>();

        html! {
            <>
                <h2>{format!("{}", task_view.holder)}</h2>
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
