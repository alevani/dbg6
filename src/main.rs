use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

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

struct TaskView {}

impl Component for TaskView {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let task_datas = get_tasks(4_usize, 12);

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
}

struct Application {}

impl Component for Application {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "DBG6 CLEANING"}</h1>
                <div>
                    <TaskView/>
                </div>
            </>
        }
    }
}


pub fn main() {
    App::<Application>::new().mount_to_body();
}