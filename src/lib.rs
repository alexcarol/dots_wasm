extern crate itertools_num;
#[macro_use]
extern crate lazy_static;

mod controllers;
mod game_state;
mod geometry;
mod models;

use std::os::raw::{c_double, c_int};
use std::sync::Mutex;

use self::game_state::GameState;
use self::geometry::Size;
use self::controllers::{Actions, TimeController};

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data(1024.0, 600.0));
}

struct GameData {
    state: GameState,
    actions: Actions,
    time_controller: TimeController,
}

fn new_game_data(width: f64, height: f64) -> GameData {
    GameData {
        state: GameState::new(Size::new(width, height)),
        actions: Actions::default(),
        time_controller: TimeController::new(),
    }
}

// These functions are provided by the runtime
extern "C" {
    fn clear_screen();
    fn draw_enemy(_: c_double, _: c_double);
    fn draw_particle(_: c_double, _: c_double, _: c_double);
    fn draw_score(_: c_double);
    fn draw_line(_: c_double, _: c_double, _: c_double, _: c_double);
    fn rust_log(_: c_int);
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
    *DATA.lock().unwrap() = new_game_data(width, height);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    let data = &mut DATA.lock().unwrap();

    clear_screen();

    draw_score(data.state.score as f64);

    if data.state.current_line_active {
        let a = &data.state.current_line.a;
        let b = &data.state.current_line.b;
        draw_line(a.point.x, a.point.y, b.point.x, b.point.y);
    }

    for row in &data.state.world.dots {
        for dot in row {
            draw_enemy(dot.point.x, dot.point.y);
        }
    }

    for line in &data.state.world.lines {
        draw_line(line.a.point.x, line.a.point.y, line.b.point.x, line.b.point.y);
    }
/*
    if data.actions.click != (0, 0) {
        rust_log(1);
    }
    if data.state.current_line_active && data.actions.mouse_position != (0, 0) {
        rust_log(2);
    }
    if data.actions.mouseup {
        rust_log(3);
    }*/
}

#[no_mangle]
pub extern "C" fn update(time: c_double) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller.update_seconds(time, &data.actions, &mut data.state);
    data.actions.mouseup = false;
    data.actions.click = (0.0, 0.0);
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}

#[no_mangle]
pub extern "C" fn toggle_shoot(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.shoot = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_boost(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.boost = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_turn_left(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_left = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn toggle_turn_right(b: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_right = int_to_bool(b);
}

#[no_mangle]
pub extern "C" fn handle_mousedown(x: c_double, y: c_double) {
    let data = &mut DATA.lock().unwrap();
    data.actions.click = (x, y);
}

#[no_mangle]
pub extern "C" fn handle_mousemove(x: c_double, y: c_double) {
    let data = &mut DATA.lock().unwrap();
    data.actions.mouse_position = (x, y);
}

#[no_mangle]
pub extern "C" fn handle_mouseup() {
    let data = &mut DATA.lock().unwrap();
    data.actions.mouseup = true
}
