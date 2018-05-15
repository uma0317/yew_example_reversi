extern crate yew;
extern crate Reversi;

use yew::prelude::*;
use Reversi::Game;

fn main() {
    yew::initialize();
    let app: App<_, Game> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}