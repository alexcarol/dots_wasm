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
    static ref DATA: Mutex<GameData<'a>> = Mutex::new(new_game_data(1024.0, 600.0));
}

struct GameData<'a> {
    state: GameState<'a>,
    actions: Actions,
    time_controller: TimeController,
}

fn new_game_data<'a>(width: f64, height: f64) -> &'static GameData<'a> {
    &GameData {
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
    fn draw_line(_: c_int, _: c_int, _: c_int, _: c_int);
    fn rust_log(_: c_int);
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
    *DATA.lock().unwrap() = new_game_data(width, height);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    use geometry::{ Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;

    clear_screen();
    for particle in &world.particles {
        draw_particle(particle.x(), particle.y(), 5.0 * particle.ttl);
    }

    for enemy in &world.enemies {
        draw_enemy(enemy.x(), enemy.y());
    }

    draw_score(data.state.score as f64);

    if data.state.current_line_active {
        let a = &data.state.current_line.a;
        let b = &data.state.current_line.b;
        draw_line(a.x, a.y, b.x, b.y);
    }

    for line in &data.state.lines {
        draw_line(line.a.x, line.a.y, line.b.x, line.b.y);
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
    data.actions.click = (0, 0);
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
pub extern "C" fn handle_mousedown(x: c_int, y: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.click = (x as i32, y as i32);
}

#[no_mangle]
pub extern "C" fn handle_mousemove(x: c_int, y: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.mouse_position = (x as i32, y as i32);
}

#[no_mangle]
pub extern "C" fn handle_mouseup(x: c_int, y: c_int) {
    let data = &mut DATA.lock().unwrap();
    data.actions.mouseup = true
}
