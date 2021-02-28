//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;
use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;
extern crate web_sys;

wasm_bindgen_test_configure!(run_in_browser);

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::custom_size(6, 6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::custom_size(6, 6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();

    let expected_universe = expected_spaceship();

    log!("From\n{}", input_universe.render());
    input_universe.tick();
    log!("To\n{}", input_universe.render());
    log!("Expected\n{}", expected_universe.render());
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
