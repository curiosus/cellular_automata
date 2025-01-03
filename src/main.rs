use macroquad::prelude::*;

#[macroquad::main("Cellular Automata")]
async fn main() {
    const SCREEN_SIZE: usize = 1600;
    request_new_screen_size(SCREEN_SIZE as f32 * 1.0, SCREEN_SIZE as f32);

    const CELL_SIZE: usize = 1;
    let mut cells = [0; SCREEN_SIZE / CELL_SIZE];
    let num_of_cells = SCREEN_SIZE / CELL_SIZE;
    let mut generations: Vec<[i32; SCREEN_SIZE / CELL_SIZE]> = Vec::new();

    let mut current_ruleset: i32 = 30;

    cells[800] = 1;

    generations.append(&mut vec![cells.clone()]);

    loop {
        clear_background(WHITE);

        let mut y = 0.0;
        for generation in generations.iter() {
            for i in 0..num_of_cells {
                let mut color: Color = WHITE;
                if generation[i] == 0 {
                    color = BLACK;
                } else if generation[i] == 1 {
                    color = RED;
                }

                let x = (i as f32) * CELL_SIZE as f32;
                draw_rectangle(x, y, CELL_SIZE as f32, CELL_SIZE as f32, color);
            }
            y = y + CELL_SIZE as f32;
        }

        let mut next_cells = cells.clone();
        for i in 1..cells.len() - 1 {
            let left = cells[i - 1];
            let center = cells[i];
            let right = cells[i + 1];
            let new_state = rule_set(left, center, right, current_ruleset);
            next_cells[i] = new_state;
        }
        cells = next_cells;
        generations.append(&mut vec![cells.clone()]);


        next_frame().await
    }
}

fn rule_set(left: i32, center: i32, right: i32, ruleset: i32) -> i32 {
    if ruleset == 90 {
        match (left, center, right) {
            (1, 1, 1) => 0,
            (1, 1, 0) => 1,
            (1, 0, 1) => 0,
            (1, 0, 0) => 1,
            (0, 1, 1) => 1,
            (0, 1, 0) => 0,
            (0, 0, 1) => 1,
            (0, 0, 0) => 0,
            _ => 2,
        }
    } else if ruleset == 250 {
        match (left, center, right) {
            (1, 1, 1) => 1,
            (1, 1, 0) => 1,
            (1, 0, 1) => 1,
            (1, 0, 0) => 1,
            (0, 1, 1) => 1,
            (0, 1, 0) => 0,
            (0, 0, 1) => 1,
            (0, 0, 0) => 0,
            _ => 2,
        }
    } else if ruleset == 110 {
        match (left, center, right) {
            (1, 1, 1) => 0,
            (1, 1, 0) => 1,
            (1, 0, 1) => 1,
            (1, 0, 0) => 0,
            (0, 1, 1) => 1,
            (0, 1, 0) => 1,
            (0, 0, 1) => 1,
            (0, 0, 0) => 0,
            _ => 2,
        }
    } else if ruleset == 30 {
        match (left, center, right) {
            (1, 1, 1) => 0,
            (1, 1, 0) => 0,
            (1, 0, 1) => 0,
            (1, 0, 0) => 1,
            (0, 1, 1) => 1,
            (0, 1, 0) => 1,
            (0, 0, 1) => 1,
            (0, 0, 0) => 0,
            _ => 2,
        }
    } else {
        2
    }
}

