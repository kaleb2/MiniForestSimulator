use std::{collections::HashSet, hash::Hash};

use macroquad::{prelude::*, rand};

const SQUARES: i32 = 64;
const MAX_FAST_AGE: i32 = 3;
const MAX_SLOW_AGE: i32 = 11;
const MAX_BURNING_AGE: i32 = 7;
const SETUP_COUNT: usize = 8;

type Point = (i32, i32);

// an enum for the allowed states of a BoardCell
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TreeState {
    SlowGrowing,
    FastGrowing,
    Burning,
    Burned
}

// a struct for the data contained in a cell
// age is a number to be decremented each turn. 
// tree_state is a TreeState indicating what kind of tree is present:
// SlowGrowing, FastGrowing, Burning, or Burned
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct BoardCell {
    age: i32,
    tree_state: TreeState,
}

impl BoardCell {

    // creating a new BoardCell should take a state
    // or type of tree to occupy the space
    // and a max age, which will dictate how long
    // the cell remains in that state without intervention
    fn new(max_age: i32, tree_state: TreeState) -> Self {
        Self {
            age: max_age,
            tree_state,
        }
    }

    // a function to produce a set of new BoardCell spawning from this BoardCell
    // position is for the current position of this BoardCell
    // is_space_clear is a closure to determine if a randomly generated (x, y) position is clear and available or cannot be used
    // All of the rules can be different depending on the tree_state and scenario
    fn create_new_gen<F: FnMut(Point) -> bool>(&self, position: Point, is_space_clear: &mut F) -> HashSet<(Point, Self)> {

        let new_position: Option<(i32, i32)> = generate_valid_position_in_range(position, 6, is_space_clear);
        if self.tree_state == TreeState::SlowGrowing {
            match new_position {
                Some(p) => HashSet::from([(p, Self::new(MAX_SLOW_AGE, self.tree_state))]),
                None => HashSet::new(),
            }
        } else if self.tree_state == TreeState::FastGrowing {
            let mut next_gen: HashSet<(Point, Self)> = HashSet::new();
            for _ in 0..3 {
                let new_position: Option<(i32, i32)> = generate_valid_position_in_range(position, 10, is_space_clear);
                match new_position {
                    Some(p) => next_gen.insert((p, Self::new(MAX_FAST_AGE, self.tree_state))),
                    None => continue,
                };
            }
            next_gen
        } else if self.tree_state == TreeState::Burning {
            let mut next_gen: HashSet<(Point, Self)> = HashSet::new();
            for _ in 0..6 {
                let new_position: Option<(i32, i32)> = generate_valid_position_in_range(position, 2, is_space_clear);
                match new_position {
                    Some(p) => next_gen.insert((p, Self::new(self.age-1, self.tree_state))),
                    None => continue,
                };
            }
            next_gen
        } 
        else {
            HashSet::new()
        }
    }
}

// a function for generating a valid position for a new BoardCell
// position is the anchor position, assumed to be an occupied BoardCell
// range is the maximum allowed distance from the anchor position
// is_space_clear is a closure for determining if the generated space is available
// if the randomly generated position is out of bounds, the anchor position, or not clear return None
// otherwise return Some(position)
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

// a helper function to draw the board background and grid. Is used in a couple of places
fn draw_board_background(game_size: f32, offset_x: f32, offset_y: f32, sq_size: f32) {

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

// a helper function to draw the BoardCell at a position. This also handles mapping the tree_state to the color
fn draw_tree(position: Point, tree_state: TreeState, offset_x: f32, offset_y: f32, sq_size: f32) {
    // draw tree as a green circle
    let color: Color = match tree_state {
        TreeState::SlowGrowing => DARKGREEN,
        TreeState::FastGrowing => GREEN,
        TreeState::Burning => ORANGE,
        TreeState::Burned => BLACK,
    };
    draw_circle(
        offset_x + position.0 as f32 * sq_size + (sq_size / 2.0),
        offset_y + position.1 as f32 * sq_size + (sq_size / 2.0),
        sq_size / 2.0,
        color,
    );
}

// a helper function for getting the current mouse position and translating that to a board index
// used anywhere that user input is required to place a BoardCell
fn get_board_position_from_mouse_position(offset_x: f32, offset_y: f32, sq_size: f32) -> (i32, i32) {
    let (mouse_x, mouse_y) = mouse_position();
    println!("mouse: {} {}", mouse_x, mouse_y);

    let position_x: i32 = ((mouse_x - (sq_size / 2.0) - offset_x) / sq_size) as i32;
    let position_y: i32 = ((mouse_y - (sq_size / 2.0) - offset_y) / sq_size) as i32;

    (position_x, position_y)
}

#[macroquad::main("Mini Forest Sim")]
async fn main() {
    // this tracks the state of the entire board
    //      Some(BoardCell) means there is something there. Otherwise, None
    let mut board: [[Option<BoardCell>; SQUARES as usize]; SQUARES as usize] = [[Option::None; SQUARES as usize]; SQUARES as usize];
    let mut tree_count: usize = 0;
    let update_period = 1.0;
    let mut last_update: f64 = get_time();
    let mut end_sim: bool = false;
    // these sets need to be tracked outside of the game state update algorithm
    // they will be updated with user input, and game state changes
    // they must be cleared within the state update code
    let mut positions_to_burning: HashSet<(Point, BoardCell)> = HashSet::new();
    let mut new_trees: HashSet<(Point, BoardCell)> = HashSet::new();

    loop {
        let game_size: f32 = screen_width().min(screen_height());
        let offset_x: f32 = (screen_width() - game_size) / 2. + 10.;
        let offset_y: f32 = (screen_height() - game_size) / 2. + 10.;
        let sq_size: f32 = (screen_height() - offset_y * 2.) / SQUARES as f32;
        // this initial_trees vec is only used to help display the user input initially
        // after the setup loop ends, it can be ignored.
        let mut initial_trees: Vec<(Point, TreeState)> = vec![];

        // while loop for handling the setup of the sim
        while tree_count < SETUP_COUNT {
            draw_board_background(game_size, offset_x, offset_y, sq_size);
            draw_text(
                format!("Tree count {tree_count} Click to add more trees up to {SETUP_COUNT}").as_str(),
                10.,
                20.,
                20.,
                BLACK,
            );

            if is_mouse_button_pressed(MouseButton::Left) {
                let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

                println!("slow tree position: {} {}", position_x, position_y);

                let position: &mut Option<BoardCell> = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
                *position = Some(BoardCell::new(MAX_SLOW_AGE, TreeState::SlowGrowing));
                initial_trees.push(((position_x, position_y), TreeState::SlowGrowing));
                tree_count += 1;
            } else if is_mouse_button_pressed(MouseButton::Right) {
                let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

                println!("fast tree position: {} {}", position_x, position_y);

                let position: &mut Option<BoardCell> = board.get_mut(position_x as usize).unwrap().get_mut(position_y as usize).unwrap();
                *position = Some(BoardCell::new(MAX_FAST_AGE, TreeState::FastGrowing));
                initial_trees.push(((position_x, position_y), TreeState::FastGrowing));
                tree_count += 1;
            }
            
            for tree in &initial_trees {
                draw_tree(tree.0, tree.1, offset_x, offset_y, sq_size);
            }

            next_frame().await;
        }
        // handle quit
        if !end_sim && is_key_down(KeyCode::Q) {
            end_sim = true;
        }
        // handle user input for BoardCells
        if is_mouse_button_pressed(MouseButton::Middle) {
            let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

            println!("fire position: {} {}", position_x, position_y);

            positions_to_burning.insert(((position_x, position_y), BoardCell::new(MAX_BURNING_AGE, TreeState::Burning)));

        } else if is_mouse_button_pressed(MouseButton::Left) {
            let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

            println!("slow tree position: {} {}", position_x, position_y);

            new_trees.insert(((position_x, position_y), BoardCell::new(MAX_SLOW_AGE, TreeState::SlowGrowing)));

        } else if is_mouse_button_pressed(MouseButton::Right) {
            let (position_x, position_y) = get_board_position_from_mouse_position(offset_x, offset_y, sq_size);

            println!("fast tree position: {} {}", position_x, position_y);

            new_trees.insert(((position_x, position_y), BoardCell::new(MAX_FAST_AGE, TreeState::FastGrowing)));
        }
        // this block updates the game state
        if !end_sim && get_time() - last_update > update_period && tree_count < 4000 {

            last_update = get_time();
            let mut positions_to_burned: HashSet<Point> = HashSet::new();
            let mut positions_to_none: HashSet<Point> = HashSet::new();            

            // iterate through the board tracking various sets of state changes
            for i in 0..SQUARES {
                for j in 0..SQUARES {
                    let cell_option: &mut Option<BoardCell> = board.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap();
                    if cell_option.is_none() {
                        continue;
                    }
                    let cell: BoardCell = cell_option.unwrap();
                    
                    // update the cell with a decremented age
                    if cell.age > 0 {
                        *cell_option = Some(BoardCell::new(cell.age -1, cell.tree_state));
                    }

                    // handle slow and fast trees. First dying, then creating more
                    if cell.tree_state == TreeState::SlowGrowing || cell.tree_state == TreeState::FastGrowing {
                        if cell.age == 0 {
                            positions_to_none.insert((i, j));
                            // handle big tree falls and nurse logs
                            if cell.tree_state == TreeState::SlowGrowing {
                                let other_fallen_positions: HashSet<Point> = match rand::gen_range(1, 5) {
                                    1 => HashSet::from([(i, j+1), (i, j+2), (i, j+3), (i, j+4), (i, j+5), (i, j+6)]),
                                    2 => HashSet::from([(i+1, j), (i+2, j), (i+3, j), (i+4, j), (i+5, j), (i+6, j)]),
                                    3 => HashSet::from([(i, j-1), (i, j-2), (i, j-3), (i, j-4), (i, j-5), (i, j-6)]),
                                    4 => HashSet::from([(i-1, j), (i-2, j), (i-3, j), (i-4, j), (i-5, j), (i-6, j)]),
                                    _ => HashSet::from([]),
                                };
                                for pos in other_fallen_positions {
                                    let rand = rand::gen_range(0, 10);
                                    if rand < 5 {
                                        new_trees.insert((pos, BoardCell::new(MAX_SLOW_AGE, TreeState::SlowGrowing)));
                                    } else if rand > 8 {
                                        new_trees.insert((pos, BoardCell::new(MAX_FAST_AGE, TreeState::FastGrowing)));
                                    }
                                    else {
                                        positions_to_none.insert(pos);
                                    }
                                }
                            }
                        } else {
                            let sapplings: HashSet<(Point, BoardCell)> = cell.create_new_gen((i, j), &mut |p: (i32, i32)| board.get(p.0 as usize).unwrap().get(p.1 as usize).unwrap().is_none());
                            for sap in sapplings  {
                                new_trees.insert(sap);
                            }
                        }
                    // handle fires burning. First dying, then spreading
                    } else if cell.tree_state == TreeState::Burning {
                        if cell.age == 0 {
                            positions_to_burned.insert((i, j));
                        } else {
                            // ensure the fire can only spread to cells with a Fast or Slow tree "burnable"
                            let more_fires: HashSet<(Point, BoardCell)> = cell.create_new_gen((i, j), &mut |p: (i32, i32)| {
                                    let cell: &Option<BoardCell> = board.get(p.0 as usize).unwrap().get(p.1 as usize).unwrap();
                                    cell.is_some() && (cell.unwrap().tree_state == TreeState::FastGrowing || cell.unwrap().tree_state == TreeState::SlowGrowing)
                                });

                            for fire in more_fires  {
                                positions_to_burning.insert(fire);
                            }
                        }
                    // handle burned fires going to None
                    } else if cell.tree_state == TreeState::Burned {
                        positions_to_none.insert((i, j));

                        // handle pioneer species
                        if rand::gen_range(0, 10) < 1 {
                            new_trees.insert(((i, j), BoardCell::new(MAX_FAST_AGE, TreeState::FastGrowing)));
                        }
                    }
                }
            }

            // iterate through each state change set and apply it to the board in priority order.
            // first fire spreading
            for burning in &positions_to_burning {
                if board.get_mut(burning.0.0 as usize).is_some() && board.get_mut(burning.0.0 as usize).unwrap().get_mut((burning.0.1) as usize).is_some() {
                    let cell_option: &mut Option<BoardCell> = board.get_mut(burning.0.0 as usize).unwrap().get_mut(burning.0.1 as usize).unwrap();
                    
                    if cell_option.is_some() && (cell_option.unwrap().tree_state == TreeState::SlowGrowing || cell_option.unwrap().tree_state == TreeState::FastGrowing) {
                        
                        *cell_option = Some(burning.1);
                    }
                }                
            }
            positions_to_burning.clear();

            // then fires dying
            for burned in positions_to_burned {
                let cell_option: &mut Option<BoardCell> = board.get_mut(burned.0 as usize).unwrap().get_mut(burned.1 as usize).unwrap();
                if cell_option.is_some() {

                    *cell_option = Some(BoardCell::new(1, TreeState::Burned));
                }
            }

            // then spaces cleared 
            for none in positions_to_none {
                if board.get_mut(none.0 as usize).is_some() && board.get_mut(none.0 as usize).unwrap().get_mut((none.1) as usize).is_some() {

                    let cell_option: &mut Option<BoardCell> = board.get_mut(none.0 as usize).unwrap().get_mut(none.1 as usize).unwrap();
                    *cell_option = None;
                }
            }

            // finally new trees 
            for new_tree in &new_trees  {
                if board.get_mut(new_tree.0.0 as usize).is_some() && board.get_mut(new_tree.0.0 as usize).unwrap().get_mut((new_tree.0.1) as usize).is_some() {
                    let cell_option: &mut Option<BoardCell> = board.get_mut(new_tree.0.0 as usize).unwrap().get_mut(new_tree.0.1 as usize).unwrap();

                    // still need to check that the cell is not burning or burned
                    if cell_option.is_some() && (cell_option.unwrap().tree_state == TreeState::Burning || cell_option.unwrap().tree_state == TreeState::Burned) {
                        continue;
                    }
                    else {
                        *cell_option = Some(new_tree.1);
                    }
                }                
            }
            new_trees.clear();
        }
        // this block updates the screen state
        if !end_sim {

            draw_board_background(game_size, offset_x, offset_y, sq_size);
            // iterate once more through the board to display the trees
            tree_count = 0;
            for i in 0..SQUARES {
                for j in 0..SQUARES {
                    let cell_option: &mut Option<BoardCell> = board.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap();
                    if cell_option.is_none() {
                        continue;
                    }
                    let cell = cell_option.unwrap();
                    if cell.tree_state == TreeState::SlowGrowing || cell.tree_state == TreeState::FastGrowing {
                        tree_count += 1;
                    } 
                    draw_tree((i, j), cell.tree_state, offset_x, offset_y, sq_size);
                }
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
                tree_count = 0;
                end_sim = false;
                board = [[Option::None; SQUARES as usize]; SQUARES as usize];
            }
        }
        next_frame().await;
    }
}
