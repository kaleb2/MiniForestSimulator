# MiniForestSimulator
This is a Rust program for simulating a very small subset of old growth forest behaviors.


## Run

```rust
cargo run
```

## Features

[x] Trees that multiply

[x] Trees that die

[x] Trees that are removed when dead

[x] Dead trees create nurse logs

[x] Differentiate big slow growing trees and smaller quick growing trees

[x] Removing trees due to burning

## Internal Features

[x] helper function to update the display text
[x] helper function to redraw the board

## Known Bugs

[x] new generations of trees are able to be generated in the same location as the parent
[x] new generations of trees are able to be generated with a location outside the grid
[x] new generations of trees are able to be generated in a location where a tree already exists
[x] tree count does not update in the UI
[x] restarting does not restart the state
[x] board is not working properly for valid tree placement

## Design Notes

## Acknowledgments
Used the following text for learning rust

*Programming Rust: Fast, Safe Systems Development
Jim Blandy and Jason Orendorff
2nd edition, O'Reilly 2021*

Using macroquad game engine
referenced https://macroquad.rs/examples/ snake.rs as a starting point

## License 

See attached MIT License