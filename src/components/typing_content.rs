use yew::prelude::*;

pub struct TypingContent {
    text: String,
}

pub enum Msg {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub text: String,
}

// var TxtRotate = function(el, toRotate, period) {
//   this.toRotate = toRotate;
//   this.el = el;
//   this.loopNum = 0;
//   this.period = parseInt(period, 10) || 2000;
//   this.txt = '';
//   this.tick();
//   this.isDeleting = false;
// };
//
// TxtRotate.prototype.tick = function() {
//   var i = this.loopNum % this.toRotate.length;
//   var fullTxt = this.toRotate[i];
//
//   if (this.isDeleting) {
//     this.txt = fullTxt.substring(0, this.txt.length - 1);
//   } else {
//     this.txt = fullTxt.substring(0, this.txt.length + 1);
//   }
//
//   this.el.innerHTML = '<span class="wrap">'+this.txt+'</span>';
//
//   var that = this;
//   var delta = 300 - Math.random() * 100;
//
//   if (this.isDeleting) { delta /= 2; }
//
//   if (!this.isDeleting && this.txt === fullTxt) {
//     delta = this.period;
//     this.isDeleting = true;
//   } else if (this.isDeleting && this.txt === '') {
//     this.isDeleting = false;
//     this.loopNum++;
//     delta = 500;
//   }
//
//   setTimeout(function() {
//     that.tick();
//   }, delta);
// };
//
impl Component for TypingContent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TypingContent { text: props.text }
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
            <p class="typing-parent">
                { &self.text }
                <div class="typing-border" />
            </p>
        }
    }
}
