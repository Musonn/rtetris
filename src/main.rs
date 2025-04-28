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

    {
        let board = board.clone();
        use_effect(move || {
            // Access the global `window` object
            let window = window().expect("no global `window` exists");
            
            // Create a keyboard event listener
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

            // Cleanup function to remove the event listener
            move || drop(listener)
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