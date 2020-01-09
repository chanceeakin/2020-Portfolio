#![recursion_limit = "128"]

use core::time::Duration;
use stdweb::web::Date;

mod components;

use self::components::title::Title;
use yew::services::{ConsoleService, IntervalService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    interval: IntervalService,
    console: ConsoleService,
    callback_tick: Callback<()>,
    date: Date,
    job: Option<Box<dyn Task>>,
    _standalone: Box<dyn Task>,
}

pub enum Msg {
    StartInterval,
    Cancel,
    Tick,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // This callback doesn't send any message to a scope
        let callback = |_| {
            println!("Example of a standalone callback.");
        };
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_secs(10), callback.into());

        Model {
            link: link.clone(),
            interval,
            console: ConsoleService::new(),
            callback_tick: link.callback(|_| Msg::Tick),
            job: None,
            _standalone: Box::new(handle),
            date: Date::new(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        {
            let handle = self
                .interval
                .spawn(Duration::from_secs(1), self.callback_tick.clone());
            self.job = Some(Box::new(handle));
        }
        self.date = Date::new();
        true
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartInterval => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                {
                    let handle = self
                        .interval
                        .spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                self.date = Date::new();
                self.console.clear();
                self.console.log("Interval started!");
            }
            Msg::Cancel => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                self.console.warn("Canceled!");
                self.console.assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Tick => {
                self.console.count_named("Tick");
                self.date = Date::new();
            }
        }
        true
    }
    fn view(&self) -> Html {
        let has_job = self.job.is_some();
        html! {
            <div>
                <Title text="Chance Eakin" />
                <p>{ self.date.to_string() }</p>
                <button disabled=has_job
                        onclick=self.link.callback(|_| Msg::StartInterval)>{ "Start time" }</button>
                <button disabled=!has_job
                        onclick=self.link.callback(|_| Msg::Cancel)>{ "Freeze time" }</button>
            </div>
        }
    }
}
