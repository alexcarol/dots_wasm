use std::f64;

use super::Actions;
use game_state::GameState;
use geometry::{Collide};
use models::{Mouse};

/// Timers to handle creation of bullets, enemies and particles
pub struct TimeController {
    current_time: f64,
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController {
            current_time: 0.0,
        }
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds(&mut self, dt: f64, actions: &Actions, state: &mut GameState) {
        self.current_time += dt;

        if actions.click != (0, 0) {
            let mouse = Mouse::new(actions.click.0, actions.click.1);

            for row in &state.world.dots {
                for dot in row {
                    if dot.collides_with(mouse) {
                        state.current_line_active = true;
                        state.current_line.a = dot;

                        break;
                    }
                }
            }

        }

        if state.current_line_active && actions.mouse_position != (0, 0) {
            state.current_line.b.x = actions.mouse_position.0;
            state.current_line.b.y = actions.mouse_position.1;
        }

        if actions.mouseup {
            state.current_line_active = false;

            let mouse = Mouse::new(state.current_line.b.x, state.current_line.b.y);

            for enemy in &state.world.enemies {
                if enemy.collides_with(&mouse) {
                    state.current_line.b.x = enemy.position().x as i32;
                    state.current_line.b.y =  enemy.position().y as i32;
                    state.lines.push(state.current_line);

                    break;
                }
            }
        }


       /* might be useful to detect where the mouse is
        // Spawn enemies at random locations
        if self.current_time - self.last_spawned_enemy > ENEMY_SPAWN_RATE {
            self.last_spawned_enemy = self.current_time;

            let player_pos: &Vector = &state.world.player.vector;
            let mut enemy_pos;
            // We loop here, just in case the new enemy random position is exactly equal
            // to the players current position, this would break our calculations below

            // Check if the newly spawned enemy is inside the player's grace area,
            // if so, we push its spawn point to the edge of the area
            if enemy_pos.position.intersect_circle(&player_pos.position, PLAYER_GRACE_AREA) {
                let length: f64 = enemy_pos.position.squared_distance_to(&player_pos.position).sqrt();
                let dp: Point = enemy_pos.position - player_pos.position;
                enemy_pos.position = player_pos.position + dp / length * PLAYER_GRACE_AREA;
            }
        }*/
    }
}
