use macroquad::{prelude::*, rand};

const SQUARES: i32 = 128;

type Point = (i32, i32);

struct Tree {
    position: Point,
    _size: i32,
    color: Color,
}

impl Tree {
    fn new(position_x: i32, position_y: i32) -> Self {
        Self {
            position: (position_x, position_y),
            _size: 1,
            color: GREEN,
        }
    }
}

trait Plant {
    fn create_new_gen(&self) -> Self;
}

fn generate_valid_nonzero_offset_in_range(range: i32) -> i32 {
    let mut offset: i32 = 0;
    while offset == 0 {
        offset = rand::gen_range(-range, range);
    }
    offset
}

impl Plant for Tree {
    fn create_new_gen(&self) -> Self {
        let new_position_x: i32 = generate_valid_nonzero_offset_in_range(6);
        let new_position_y: i32 = generate_valid_nonzero_offset_in_range(6);
        Self {
            position: (
                self.position.0 + new_position_x,
                self.position.1 + new_position_y,
            ),
            _size: 1,
            color: GREEN,
        }
    }
}

#[macroquad::main("Mini Forest Sim")]
async fn main() {
    let mut trees: Vec<Tree> = vec![];
    let mut tree_count: usize = 0;
    let update_period = 1.0;
    let mut last_update: f64 = get_time();
    let mut end_sim: bool = false;

    loop {
        let game_size: f32 = screen_width().min(screen_height());
        let offset_x: f32 = (screen_width() - game_size) / 2. + 10.;
        let offset_y: f32 = (screen_height() - game_size) / 2. + 10.;
        let sq_size: f32 = (screen_height() - offset_y * 2.) / SQUARES as f32;

        while trees.len() < 3 {
            clear_background(LIGHTGRAY);

            // draw background
            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            // draw horizontal lines to form boxes
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            // draw vertical lines to form boxes
            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            draw_text(
                format!("Tree count {tree_count} Click to add more trees").as_str(),
                10.,
                20.,
                20.,
                BLACK,
            );

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                println!("mouse: {} {}", mouse_x, mouse_y);

                let position_x: f32 = (mouse_x - (sq_size / 2.0) - offset_x) / sq_size;
                let position_y: f32 = (mouse_y - (sq_size / 2.0) - offset_y) / sq_size;

                println!("position: {} {}", position_x, position_y);

                trees.push(Tree::new(position_x as i32, position_y as i32));
            }

            for tree in &trees {
                // draw tree as a green circle
                draw_circle(
                    offset_x + tree.position.0 as f32 * sq_size + (sq_size / 2.0),
                    offset_y + tree.position.1 as f32 * sq_size + (sq_size / 2.0),
                    sq_size / 2.0,
                    // sq_size,
                    tree.color,
                );
            }

            next_frame().await;
        }
        // handle quit
        if !end_sim && is_key_down(KeyCode::Q) {
            end_sim = true;
        }
        // // this block updates the game state
        if !end_sim && get_time() - last_update > update_period && trees.len() < 4000 {
            println!("Updating trees");

            last_update = get_time();
            let mut new_trees: Vec<Tree> = vec![];
            for tree in &trees {
                new_trees.push(tree.create_new_gen());
            }
            for tree in new_trees {
                trees.push(tree);
            }
            tree_count = trees.len();
        }
        // this block updates the screen state
        if !end_sim {
            clear_background(LIGHTGRAY);

            // draw background
            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            // draw horizontal lines to form boxes
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            // draw vertical lines to form boxes
            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            for tree in &trees {
                // draw tree as a green circle
                draw_circle(
                    offset_x + tree.position.0 as f32 * sq_size + (sq_size / 2.0),
                    offset_y + tree.position.1 as f32 * sq_size + (sq_size / 2.0),
                    sq_size / 2.0,
                    // sq_size,
                    tree.color,
                );
            }

            // draw instructions
            draw_text(
                format!("Tree count {tree_count} Press Q to quit").as_str(),
                10.,
                20.,
                20.,
                BLACK,
            );
        // this block draws game over
        } else {
            // leave the screen
            clear_background(WHITE);
            let text: &str = "Simulation ended. Press [enter] to play again.";
            let font_size: f32 = 30.;
            let text_size: TextDimensions = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                BLACK,
            );

            if is_key_down(KeyCode::Enter) {
                trees.clear();
                tree_count = 0;
                end_sim = false;
            }
        }
        next_frame().await;
    }
}
