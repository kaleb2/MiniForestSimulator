use macroquad::{prelude::*, rand};

const SQUARES: i32 = 64;

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TreeType {
    SlowGrowing,
    FastGrowing,
    Burning,
}

#[derive(Clone, Debug)]
struct Tree {
    position: Point,
    age: i32,
    color: Color,
    tree_type: TreeType,
    is_dead: bool,
}

impl Tree {
    fn new(position_x: i32, position_y: i32, tree_type: TreeType) -> Self {
        Self {
            position: (position_x, position_y),
            age: 0,
            color: match tree_type {
                        TreeType::FastGrowing => GREEN,
                        TreeType::SlowGrowing => DARKGREEN,
                        TreeType::Burning => ORANGE,
                    },
            tree_type,
            is_dead: false,
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
                let new_position: Option<(i32, i32)> = generate_valid_position_in_range(self.position, 10, is_space_clear);
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
            TreeType::FastGrowing => self.age >= 2 || self.is_dead,
            TreeType::SlowGrowing => self.age >= 10 || self.is_dead,
            TreeType::Burning => self.age >= 5,
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

fn draw_board(game_size: f32, offset_x: f32, offset_y: f32, sq_size: f32) {

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
}

fn draw_trees(trees: &Vec<Tree>, offset_x: f32, offset_y: f32, sq_size: f32) {
    for tree in trees {
        // draw tree as a green circle
        draw_circle(
            offset_x + tree.position.0 as f32 * sq_size + (sq_size / 2.0),
            offset_y + tree.position.1 as f32 * sq_size + (sq_size / 2.0),
            sq_size / 2.0,
            // sq_size,
            tree.color,
        );
    }
}

fn get_board_position_from_mouse_position(offset_x: f32, offset_y: f32, sq_size: f32) -> (i32, i32) {
    let (mouse_x, mouse_y) = mouse_position();
    println!("mouse: {} {}", mouse_x, mouse_y);

    let position_x: i32 = ((mouse_x - (sq_size / 2.0) - offset_x) / sq_size) as i32;
    let position_y: i32 = ((mouse_y - (sq_size / 2.0) - offset_y) / sq_size) as i32;

    (position_x, position_y)
}

#[macroquad::main("Mini Forest Sim")]
async fn main() {
    let mut trees: Vec<Tree> = vec![];
    let mut board: Vec<Vec<Option<Tree>>> = vec![vec![Option::None; SQUARES as usize]; SQUARES as usize];
    let mut tree_count: usize = 0;
    let update_period = 1.0;
    let mut last_update: f64 = get_time();
    let mut end_sim: bool = false;

    loop {
        let game_size: f32 = screen_width().min(screen_height());
        let offset_x: f32 = (screen_width() - game_size) / 2. + 10.;
        let offset_y: f32 = (screen_height() - game_size) / 2. + 10.;
        let sq_size: f32 = (screen_height() - offset_y * 2.) / SQUARES as f32;

        // innitialization code
        while trees.len() < 4 {
            draw_board(game_size, offset_x, offset_y, sq_size);
            draw_text(
                format!("Tree count {tree_count} Click to add more trees").as_str(),
                10.,
                20.,
                20.,
                BLACK,
            );

            if is_mouse_button_pressed(MouseButton::Left) {
                let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

                println!("slow tree position: {} {}", position_x, position_y);

                trees.push(Tree::new(position_x, position_y, TreeType::SlowGrowing));
                let position: &mut Option<Tree> = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
                *position = Some(Tree::new(position_x, position_y, TreeType::SlowGrowing));

            } else if is_mouse_button_pressed(MouseButton::Right) {
                let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

                println!("fast tree position: {} {}", position_x, position_y);

                trees.push(Tree::new(position_x, position_y, TreeType::FastGrowing));
                let position: &mut Option<Tree> = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
                *position = Some(Tree::new(position_x, position_y, TreeType::FastGrowing));
                
            }

            draw_trees(&trees, offset_x, offset_y, sq_size);

            next_frame().await;
        }
        // handle quit
        if !end_sim && is_key_down(KeyCode::Q) {
            end_sim = true;
        }
        if is_mouse_button_pressed(MouseButton::Middle) {
            let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

            println!("fire position: {} {}", position_x, position_y);

            trees.retain(|t: &Tree| t.position != (position_x, position_y));
            trees.push(Tree::new(position_x, position_y, TreeType::Burning));
            let position: &mut Option<Tree> = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
            *position = Some(Tree::new(position_x, position_y, TreeType::Burning));
        }
        // this block updates the game state
        if !end_sim && get_time() - last_update > update_period && trees.len() < 4000 {

            last_update = get_time();
            let mut new_trees: Vec<Tree> = vec![];
            let mut fallen_trees: Vec<(i32, i32)> = vec![];

            for tree in &mut trees {

                let sapplings: Vec<Tree> = tree.create_new_gen(&mut |p: (i32, i32)| board.get(p.0 as usize).unwrap().get(p.1 as usize).unwrap().is_none());
                for s in sapplings {
                    let position: &mut Option<Tree> = board.get_mut(s.position.0 as usize).unwrap().get_mut(s.position.1 as usize).unwrap();
                    *position = Some(Tree::new(s.position.0, s.position.1, s.tree_type));
                    new_trees.push(s);
                }
                
                if tree.is_dead() {

                    fallen_trees.push(tree.position);
                    if tree.tree_type == TreeType::SlowGrowing {
                        let mut other_fallen_positions: Vec<(i32, i32)> = match rand::gen_range(1, 5) {
                            1 => vec![(tree.position.0, tree.position.1+1), (tree.position.0, tree.position.1+2), (tree.position.0, tree.position.1+3), (tree.position.0, tree.position.1+4)],
                            2 => vec![(tree.position.0+1, tree.position.1), (tree.position.0+2, tree.position.1), (tree.position.0+3, tree.position.1), (tree.position.0+4, tree.position.1)],
                            3 => vec![(tree.position.0, tree.position.1-1), (tree.position.0, tree.position.1-2), (tree.position.0, tree.position.1-3), (tree.position.0, tree.position.1-4)],
                            4 => vec![(tree.position.0-1, tree.position.1), (tree.position.0-2, tree.position.1), (tree.position.0-3, tree.position.1), (tree.position.0-4, tree.position.1)],
                            _ => vec![],
                        };
                        fallen_trees.append(&mut other_fallen_positions);
                    }
                }
            }
            trees.append(&mut new_trees);

            for fallen_tree in &fallen_trees  {

                if board.get_mut(fallen_tree.0 as usize).is_some() && board.get_mut(fallen_tree.0 as usize).unwrap().get_mut((fallen_tree.1) as usize).is_some() {
                    let position: &mut Option<Tree> = board.get_mut(fallen_tree.0 as usize).unwrap().get_mut((fallen_tree.1) as usize).unwrap();
                    *position = Option::None;
                }
            }

            trees.retain(|t: &Tree| !t.is_dead() && !fallen_trees.contains(&t.position));

            tree_count = trees.len();
        }
        // this block updates the screen state
        if !end_sim {
            
            draw_board(game_size, offset_x, offset_y, sq_size);
            
            draw_trees(&trees, offset_x, offset_y, sq_size);

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
                board = vec![vec![Option::None; SQUARES as usize]; SQUARES as usize];
            }
        }
        next_frame().await;
    }
}
