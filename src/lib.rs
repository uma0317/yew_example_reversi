#[macro_use]
extern crate yew;

mod stone;

use stone::{Stone, State};
use yew::prelude::*;

pub struct Game {
    // state: State
}

pub enum Msg {
    Reverse,
}

impl<CTX> Component<CTX> for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Game {
            
        }
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {

        true
    }
}

fn view_stone<CTX>() -> Html<CTX, Game> 
where
    CTX: 'static
{
    html! {
        <td>
            <Stone: state=State::None,/>
        </td>
    }
}

fn view_row<CTX>() -> Html<CTX, Game> 
where
    CTX: 'static
{
    html! {
        <tr>
            {for (0..3).map(|_|{
                view_stone()
            })}
        </tr>
    }
}
impl<CTX> Renderable<CTX, Game> for Game
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <table>
                {for (0..3).map(|_| {
                    view_row()
                })}
            </table>
        }
    }
}