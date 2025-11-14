# MiniForestSimulator
This is a Rust program for simulating a very small subset of old growth forest behaviors.


## Run

```rust
cargo run
```

## Features

[x] Trees that multiply

[x] Trees that die

[ ] Trees that visually indicate age

[ ] Dead trees create nurse logs

[x] Differentiate big slow growing trees and smaller quick growing trees

[ ] Removing trees due to burning

## Internal Features

[ ] helper function to update the display text
[ ] helper function to redraw the board

## Known Bugs

[x] new generations of trees are able to be generated in the same location as the parent
[x] new generations of trees are able to be generated with a location outside the grid
[x] new generations of trees are able to be generated in a location where a tree already exists
[x] tree count does not update in the UI
[x] restarting does not restart the state
[x] board is not working properly for valid tree placement

## Design Notes

* I'm not sure that the create_new_generation function needs to be in a plant trait. It might be better instead to make it part of the tree struct impl.
    * It really comes down to where the fast vs slow growing trees are represented.
    * As a part of the Tree struct means the trait doesn't offer much
    * As a part of a different Tree struct e.g. FastTree, SlowTree with diff impls of the trait. That makes more sense, but is it necessary.

## Acknowledgments
Used the following text for learning rust

*Programming Rust: Fast, Safe Systems Development
Jim Blandy and Jason Orendorff
2nd edition, O'Reilly 2021*

Using macroquad game engine
referenced https://macroquad.rs/examples/ snake.rs as a starting point

## License 

See attached MIT License