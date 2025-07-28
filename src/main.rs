mod game;
mod tetromino;

use game::Board;
use yew::prelude::*;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use web_sys::window;
use wasm_bindgen::JsCast;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Board::new());

    // Interval to update the board every 500ms
    {
        let board = board.clone();
        use_effect(move || {
            let interval = Interval::new(500, move || {
                board.set({
                    let mut new_board = (*board).clone();
                    new_board.update();
                    new_board.clear_full_lines();
                    new_board
                });
            });

            // Cleanup interval on component unmount
            move || drop(interval)
        });
    }

    // Keyboard event listener for controlling the game
    {
        let board = board.clone();
        use_effect(move || {
            let window = window().expect("no global `window` exists");
            let listener = EventListener::new(&window, "keydown", move |event| {
                if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                    board.set({
                        let mut new_board = (*board).clone();
                        match event.key().as_str() {
                            "ArrowLeft" => {
                                new_board.clear_tetromino();
                                new_board.move_tetromino_left();
                                new_board.place_tetromino();
                            }
                            "ArrowRight" => {
                                new_board.clear_tetromino();
                                new_board.move_tetromino_right();
                                new_board.place_tetromino();
                            }
                            "ArrowDown" => {
                                new_board.clear_tetromino();
                                new_board.move_tetromino_down();
                                new_board.place_tetromino();
                            }
                            "ArrowUp" => {
                                new_board.clear_tetromino();
                                new_board.rotate_tetromino();
                                new_board.place_tetromino();
                            }
                            _ => {}
                        }
                        new_board
                    });
                }
            });

            // Cleanup listener on component unmount
            move || drop(listener)
        });
    }

    let restart = {
        let board = board.clone();
        Callback::from(move |_| {
            board.set(Board::new());
        })
    };

    html! {
        <div>
            <h1>{ "Tetris in Rust!" }</h1>
            <div class="info-panel">
                <div class="score">{ format!("Score: {}", board.get_score()) }</div>
                <div>
                    <span>{ "Next:" }</span>
                    { board.render_next_tetromino() }
                </div>
            </div>
            if board.get_game_over() {
                <div class="game-over">
                    <h2>{ "Game Over" }</h2>
                    <button onclick={restart}>{ "Restart" }</button>
                </div>
            } else {
                { board.render() }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}