use macroquad::prelude::*;

const SQUARES: i16 = 64;

type Point = (i16, i16);

struct Tree {
    position: Point,
    _size: i16,
    color: Color,
}

#[macroquad::main("Mini Forest Sim")]
async fn main() {
    let mut trees: Vec<Tree> = vec![Tree {
        position: (8, 8),
        _size: 1,
        color: GREEN,
    }];
    trees.push(Tree {
        position: (12, 8),
        _size: 1,
        color: DARKGREEN,
    });
    trees.push(Tree {
        position: (8, 9),
        _size: 1,
        color: BROWN,
    });

    let mut tree_count: i32 = 1;
    let mut end_sim: bool = false;

    loop {
        // this block updates the game state
        if !end_sim && is_key_down(KeyCode::Q) {
            end_sim = true;
        }
        // this block updates the screen state
        if !end_sim {
            clear_background(LIGHTGRAY);

            let game_size: f32 = screen_width().min(screen_height());
            let offset_x: f32 = (screen_width() - game_size) / 2. + 10.;
            let offset_y: f32 = (screen_height() - game_size) / 2. + 10.;
            let sq_size: f32 = (screen_height() - offset_y * 2.) / SQUARES as f32;

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
                DARKGRAY,
            );
        // this block draws game over
        } else {
            // leave the screen
            clear_background(WHITE);
            let text = "Simulation ended. Press [enter] to play again.";
            let font_size = 30.;
            let text_size: TextDimensions = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                for tree in trees.iter_mut() {
                    tree.position.0 += 1;
                    tree.position.1 -= 1;
                }
                tree_count = 1;
                end_sim = false;
            }
        }
        next_frame().await;
    }
}
