mod game;
mod tetromino;
use game::Board;
use yew::prelude::*;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Board::new());

    {
        let board = board.clone();
        use_effect(move || {
            // Create the interval to update the board every 500ms
            let interval = Interval::new(500, move || {
                board.set({
                    let mut new_board = (*board).clone();
                    new_board.update();
                    new_board
                });
            });

            // Cleanup function to stop the interval when the component is unmounted
            move || drop(interval)
        });
    }

    html! {
        <div>
            <h1>{ "Tetris in Rust!" }</h1>
            { board.render() }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
