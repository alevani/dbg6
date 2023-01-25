use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use gloo_console::{externs::info, info, log};
use yew::prelude::*;

pub struct TaskData {
    pub holder: String,
    pub task_section: Vec<TaskSection>,
}

pub struct TaskSection {
    pub name: String,
    pub tasks: Vec<String>,
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    number_of_participant: i32,
    task_target: i32,
}

fn callback() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn TaskView(props: &Props) -> Html {
    let task_datas = get_tasks(props.number_of_participant as usize, props.task_target);

    task_datas.iter().map(|task_data| {
        
        let ht = task_data.task_section.iter().map(|sections| {
            let inner_ht = sections.tasks.iter().map(|t_name| html! {
                <p> {format!("• {t_name}")}</p>
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
                <div class="border">
                    <h2>{format!("{}", task_data.holder)}</h2>
                    <div class="content">
                        { ht }
                    </div>
                </div>
            </>
        }
    }).collect::<Html>()
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "DBG6 CLEANING"}</h1>
            <div>
                <TaskView number_of_participant=4 task_target=12/>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
