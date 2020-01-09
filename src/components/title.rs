use yew::prelude::*;

pub struct Title {
    text: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Ignore,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub text: String,
}

impl Component for Title {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Title {
            text: props.text,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.text = props.text;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="title-container">
            <h1 class="title">
                { &self.text }
            </h1>
            </div>
        }
    }
}
