mod game;
mod tetromino;
use game::Board;
use tetromino::{Tetromino, TetrominoType};
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Board::new());
    let active_tetromino = use_state(|| Tetromino::new(TetrominoType::T));

    let move_down = {
        let tetromino = active_tetromino.clone();
        Callback::from(move |_| {
            let mut t = (*tetromino).clone();
            t.move_down();
            tetromino.set(t);
        })
    };

    html! {
        <div>
            <h1>{ "Tetris in Rust!" }</h1>
            <button onclick={move_down}>{ "Move Down" }</button>
            { board.render() }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
