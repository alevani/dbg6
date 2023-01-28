use std::{collections::HashMap, borrow::BorrowMut};

use dbgg_resources::get_tasks;

pub mod data;
pub mod dbgg_resources;

use gloo_console::info;
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
    participants: HashMap<&'static str, bool>,
    link: ComponentLink<Self>,
}

impl Component for TaskView {
    type Message = &'static str;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut p: HashMap<&'static str, bool> = HashMap::new();
        p.insert("G. Alexander", true);
        p.insert("V. Alexandre", true);
        p.insert("Henriette", true);
        p.insert("Jon", true);

        Self {
            participants: p,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.participants.entry(msg).and_modify(|status| {*status = !*status});
        
        // todo maybe optimise that
        if !self.participants.values().any(|&v| v) {
            self.participants.entry(msg).and_modify(|status| {*status = !*status});
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (a, i): (HashMap<&'static str, bool>, HashMap<&'static str, bool>) = self.participants.iter().partition(|(_,v)| *v == &true);
        let mut a_participants: Vec<&'static str> = a.keys().cloned().collect();
        let nb_active_participants = a_participants.len();
        let target = 48 / nb_active_participants;
        
        a_participants.sort(); //todo Somehow, the lists get messed up, so we sort :-)
        let task_datas = get_tasks(nb_active_participants, target as i32, &a_participants);

        let node = task_datas.into_iter().map(|task_data| {
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
                        <h2 onclick=self.link.callback(move |_| task_data.holder)>{format!("{}", task_data.holder)}</h2>
                        <div class="content">
                            { ht }
                        </div>
                    </div>
                </>
            }
        }).collect::<Html>();

        let mut keys: Vec<_> = i.keys().cloned().collect();
        keys.sort(); //todo Somehow, the lists get messed up, so we sort :-)
        let non_participating = keys.into_iter().map(|name| {
            html! {
                <div class="border disabled">
                    <h2 onclick=self.link.callback(move |_| name)>{format!("{}", name)}</h2>
                </div>
                
            }
        }).collect::<Html>();

        html! {
            <>
                { non_participating }
                { node }
            </>
        }
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