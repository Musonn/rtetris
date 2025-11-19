mod game;
mod tetromino;

use game::Board;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use std::sync::{Arc, RwLock};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

// Interval should have implemented the Sync and Send trait because it introduces parallelization, but it wasn't,
// which happens to resolve the issue that use_state is implemented with only Rc but not Arc internally.
// If you uncomment the Sync and Send trait below, the codes will no longer compile.
// F*** gloo_timers. F*** yew.
fn new_interval<F>(millis: u32, callback: F) -> Interval
where
    F: 'static /*+ Sync + Send*/ + FnMut(),
{
    Interval::new(millis, callback)
}

// Callback should have implemented the Sync and Send trait because it introduces parallelization, but it wasn't,
// which happens to resolve the issue that use_state is implemented with only Rc but not Arc internally.
// If you uncomment the Sync and Send trait below, the codes will no longer compile.
// F*** yew.
fn callback_from<IN, OUT, F: Fn(IN) -> OUT + 'static>(func: F) -> Callback<IN, OUT>
// where
//     F: Sync + Send,
{
    Callback::from(func)
}

#[function_component(App)]
fn app() -> Html {
    // Yew really should have exposed the internal Rc (or Arc, if yew was implemented correctly for parallelization),
    // so I wouldn't have to fill another Arc into use_state's Rc/Arc.
    // F*** yew.
    let board = use_state(|| Arc::new(RwLock::new(Board::new())));
    let game_over = match board.read() {
        Ok(board) => board.get_game_over(),
        Err(err) => panic!("{}", err),
    };

    // Interval to update the board every 500ms
    {
        let board = board.clone();
        use_effect_with_deps(
            move |_| {
                let interval = new_interval(500, move || {
                    board.set({
                        let new_board = (*board).clone();
                        match new_board.write() {
                            Ok(mut board) => board.update(),
                            Err(err) => panic!("{}", err),
                        };
                        new_board
                    });
                });

                // Cleanup interval on component unmount
                move || drop(interval)
            },
            game_over,
        );
    }

    // Keyboard event listener for controlling the game
    {
        let board = board.clone();
        use_effect_with_deps(
            move |_| {
                let window = window().expect("no global `window` exists");
                let listener = EventListener::new(&window, "keydown", move |event| {
                    if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                        match event.key().as_str() {
                            "ArrowLeft" => board.set({
                                let new_board = (*board).clone();
                                match new_board.write() {
                                    Ok(mut board) => {
                                        board.move_tetromino_left();
                                        board.predict_tetromino();
                                    }
                                    Err(err) => panic!("{}", err),
                                };
                                new_board
                            }),
                            "ArrowRight" => board.set({
                                let new_board = (*board).clone();
                                match new_board.write() {
                                    Ok(mut board) => {
                                        board.move_tetromino_right();
                                        board.predict_tetromino();
                                    }
                                    Err(err) => panic!("{}", err),
                                };
                                new_board
                            }),
                            "ArrowDown" => board.set({
                                let new_board = (*board).clone();
                                match new_board.write() {
                                    Ok(mut board) => {
                                        board.move_tetromino_down();
                                        board.predict_tetromino();
                                    }
                                    Err(err) => panic!("{}", err),
                                };
                                new_board
                            }),
                            "ArrowUp" => board.set({
                                let new_board = (*board).clone();
                                match new_board.write() {
                                    Ok(mut board) => {
                                        board.rotate_tetromino();
                                        board.predict_tetromino();
                                    }
                                    Err(err) => panic!("{}", err),
                                };
                                new_board
                            }),
                            _ => {}
                        }
                    }
                });

                // Cleanup listener on component unmount
                move || drop(listener)
            },
            game_over,
        );
    }

    let restart = {
        let board = board.clone();
        callback_from(move |_| {
            board.set(Arc::new(RwLock::new(Board::new())));
        })
    };

    match board.read() {
        Ok(board) => html! {
            <div>
                <h1>{ "Tetris in Rust!" }</h1>
                <div class="info-panel">
                    <div class="score">{ format!("Score: {}", board.get_score()) }</div>
                    <div>
                        <span>{ "Next:" }</span>
                        { board.render_next_tetromino() }
                    </div>
                </div>
                if game_over {
                    <div class="game-over">
                        <h2>{ "Game Over" }</h2>
                        <button onclick={restart}>{ "Restart" }</button>
                    </div>
                } else {
                    <div class="board-wrapper">
                        { board.render_static() }
                        { board.render_active() }
                    </div>
                }
            </div>
        },
        Err(err) => panic!("{}", err),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
