use yew::prelude::*;

pub struct Title {
    text: String,
}

pub enum Msg {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub text: String,
}

impl Component for Title {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Title { text: props.text }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
