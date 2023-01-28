use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use yew::prelude::*;

pub struct TaskData {
    pub holder: &'static str,
    pub task_section: Vec<TaskSection>,
}

pub struct TaskSection {
    pub name: String,
    pub tasks: Vec<String>,
}

struct TaskView {
    participants: Vec<&'static str>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Remove(&'static str),
    Add(&'static str),
}

impl Component for TaskView {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            participants: vec!["G. Alexander", "V. Alexandre", "Henriette", "Jon"],
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add(person) => {
                self.participants.push(person);
                self.participants.sort();
            },
            Msg::Remove(person) => {
                let index = self.participants.iter().position(|x| *x == person).unwrap();
                self.participants.remove(index);
            },
        } 
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let nb_participants = self.participants.len();
        let target = 48 / nb_participants;
        let task_datas = get_tasks(nb_participants, target as i32, &self.participants);

        task_datas.into_iter().map(|task_data| {
            
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
                        <h2 onclick=self.link.callback(move |_| Msg::Remove(task_data.holder))>{format!("{}", task_data.holder)}</h2>
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