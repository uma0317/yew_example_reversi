use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum State {
    Flont,
    Back,
    None
}

pub struct Stone {
    state: State
}
#[derive(PartialEq, Clone)]
pub struct Props {
    pub state: State
}

impl Default for Props {
    fn default() -> Self {
        Props {
            state: State::None
        }
    }
}
pub enum Msg {
    Reverse,
}


impl<CTX> Component<CTX> for Stone {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Stone {
            state: props.state
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Reverse => {
                if self.state == State::Flont {
                    self.state = State::Back
                } else {
                    self.state = State::Flont
                }
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Stone> for Stone
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="stone", onclick=|_| Msg::Reverse, >
            {
                if self.state == State::Flont {
                    "●"
                } else if self.state == State::Back {
                    "○"
                } else {
                    ""
                }
            }
            </div>
        }
    }
}