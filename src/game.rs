use yew::prelude::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub struct Board {
    grid: [[bool; WIDTH]; HEIGHT], // Fixed-size 2D array; true = filled
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == false
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        self.grid[y][x] = value;
    }

    pub fn clear_full_lines(&mut self) {
        let mut new_grid = [[false; WIDTH]; HEIGHT]; // Start with empty grid
        let mut new_row = HEIGHT as isize - 1; // Start from the bottom

        // Iterate from bottom to top, shifting rows down if they are not full
        for y in (0..HEIGHT).rev() {
            if self.grid[y].iter().any(|&cell| !cell) {
                new_grid[new_row as usize] = self.grid[y]; // Copy row
                new_row -= 1; // Move up
            }
        }

        self.grid = new_grid; // Update board with the cleared version
    }

pub fn render(&self) -> Html {
    html! {
        <div class="board">
            <p>{"If you see this, Yew is rendering the component."}</p>
            { for self.grid.iter().map(|row| html! {
                <div class="row">
                    { for row.iter().map(|&cell| html! {
                        <div class={ if cell { "cell filled" } else { "cell empty" } }></div>
                    }) }
                </div>
            }) }
        </div>
    }
}

}
