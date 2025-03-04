mod game;
use game::Board;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let board = use_state(|| Board::new());

    html! {
        <div>
            <h1>{ "Tetris in Rust!" }</h1>
            <p>{ "Game Board Initialized" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
