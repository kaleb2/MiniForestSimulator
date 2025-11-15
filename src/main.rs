use macroquad::{prelude::*, rand};

const SQUARES: i32 = 32;

type Point = (i32, i32);

#[derive(Clone, Copy, PartialEq, Eq)]
enum TreeType {
    SlowGrowing,
    FastGrowing,
}

struct Tree {
    position: Point,
    age: i32,
    color: Color,
    tree_type: TreeType,
}

impl Tree {
    fn new(position_x: i32, position_y: i32, tree_type: TreeType) -> Self {
        Self {
            position: (position_x, position_y),
            age: 0,
            color: match tree_type {
                        TreeType::FastGrowing => GREEN,
                        TreeType::SlowGrowing => DARKGREEN,
                    },
            tree_type
        }
    }

    fn create_new_gen<F: FnMut(Point) -> bool>(&mut self, is_space_clear: &mut F) -> Vec<Self> {

        let new_position: Option<(i32, i32)> = generate_valid_position_in_range(self.position, 6, is_space_clear);
        self.age += 1;
        if self.tree_type == TreeType::SlowGrowing {
            match new_position {
                Some(p) => vec![Self::new(p.0, p.1, self.tree_type)],
                None => vec![],
            }
        } else if self.tree_type == TreeType::FastGrowing {
            let mut next_gen: Vec<Self> = vec![];
            for _ in 0..3 {
                let new_position: Option<(i32, i32)> = generate_valid_position_in_range(self.position, 6, is_space_clear);
                match new_position {
                    Some(p) => next_gen.push(Self::new(p.0, p.1, self.tree_type)),
                    None => continue,
                }
            }
            next_gen
        } else {
            vec![]
        }        
    }

    fn is_dead(&self) -> bool{
        match self.tree_type {
            TreeType::FastGrowing => self.age >= 3,
            TreeType::SlowGrowing => self.age >= 10,
        }
    }
}

fn generate_valid_position_in_range<F: FnMut(Point) -> bool>(position: Point, range: i32, is_space_clear: &mut F) -> Option<Point> {
    let offset_x: i32 = rand::gen_range(-range, range);
    let offset_y: i32 = rand::gen_range(-range, range);
    if offset_x == 0 && offset_y == 0 {
        return None
    } 
    let position_x = position.0 + offset_x;
    let position_y = position.1 + offset_y;
    if position_x < 0 || position_y < 0 {
        return None
    }
    if position_x >= SQUARES || position_y >= SQUARES {
        return None
    }
    if !is_space_clear((position_x, position_y)) {
        return None
    }

    Some((position_x, position_y))
}

#[macroquad::main("Mini Forest Sim")]
async fn main() {
    let mut trees: Vec<Tree> = vec![];
    let mut board: Vec<Vec<bool>> = vec![vec![false; SQUARES as usize]; SQUARES as usize];
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

                trees.push(Tree::new(position_x as i32, position_y as i32, TreeType::SlowGrowing));
                let position = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
                (*position) = true;
            } else if is_mouse_button_pressed(MouseButton::Right) {
                let (mouse_x, mouse_y) = mouse_position();
                println!("mouse: {} {}", mouse_x, mouse_y);

                let position_x: f32 = (mouse_x - (sq_size / 2.0) - offset_x) / sq_size;
                let position_y: f32 = (mouse_y - (sq_size / 2.0) - offset_y) / sq_size;

                println!("position: {} {}", position_x, position_y);

                trees.push(Tree::new(position_x as i32, position_y as i32, TreeType::FastGrowing));
                let position = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
                (*position) = true;
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
            let mut dead_trees: Vec<(i32, i32)> = vec![];

            for tree in &mut trees {

                println!("tree position: ({}, {}) age: {}", tree.position.0, tree.position.1, tree.age);
                
                let sapplings: Vec<Tree> = tree.create_new_gen(&mut |p: (i32, i32)| !(*board.get(p.0 as usize).unwrap().get(p.1 as usize).unwrap()));
                for s in sapplings {
                    let position: &mut bool = board.get_mut(s.position.0 as usize).unwrap().get_mut(s.position.1 as usize).unwrap();
                    (*position) = true;
                    println!("sappling position: ({}, {}) age: {}", s.position.0, s.position.1, s.age);
                    new_trees.push(s);
                }
                
                if tree.is_dead() {
                    dead_trees.push(tree.position);
                }

                // let position = board.get_mut(tree.position.0 as usize).unwrap().get_mut(tree.position.1 as usize).unwrap();
                // (*position) = true;
            }
            trees.append(&mut new_trees);

            trees.retain(|t| !t.is_dead());

            for dead_tree in dead_trees  {
                let position = board.get_mut(dead_tree.0 as usize).unwrap().get_mut(dead_tree.1 as usize).unwrap();
                (*position) = false;
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
                board = vec![vec![false; SQUARES as usize]; SQUARES as usize];
            }
        }
        next_frame().await;
    }
}
