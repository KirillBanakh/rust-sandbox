mod hello_rust;
mod guessing_game;

use crate::hello_rust::greeting;
use crate::guessing_game::ggame;

fn main() {
    greeting::print_greeting();
    ggame::ggame_start();
}