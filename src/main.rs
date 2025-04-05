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

    let on_rotate = {
        let board = board.clone();
        Callback::from(move |_: MouseEvent| {
            board.set({
                let mut new_board = (*board).clone();
                new_board.clear_tetromino();
                new_board.rotate_tetromino();
                new_board.place_tetromino();
                new_board
            });
        })
    };

    html! {
        <div>
            <h1>{ "Tetris in Rust!" }</h1>
            <button onclick={on_rotate}>{ "Rotate Tetromino" }</button>
            { board.render() }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
