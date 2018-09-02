use game_state::GameState;

const SCORE_PER_ENEMY: u32 = 10;

pub struct CollisionsController;

impl CollisionsController {
    pub fn handle_collisions(state: &mut GameState) {
//        CollisionsController::handle_bullet_collisions(state);
//        CollisionsController::handle_player_collisions(state);
    }

    /// Handles collisions between the bullets and the enemies
    ///
    /// When an enemy is reached by a bullet, both the enemy and the bullet
    /// will be removed. Additionally, the score of the player will be increased.
    fn handle_bullet_collisions(state: &mut GameState) {
        let old_enemy_count = state.world.enemies.len();

        // We introduce a scope to shorten the lifetime of the borrows below
        {
            let enemies = &mut state.world.enemies;
            let particles = &mut state.world.particles;
        }

        let killed_enemies = (old_enemy_count - state.world.enemies.len()) as u32;
        state.score += SCORE_PER_ENEMY * killed_enemies;
    }
/*
    /// Handles collisions between the player and the enemies
    fn handle_player_collisions(state: &mut GameState) {
        if state.world.enemies.iter().any(|enemy| state.world.player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = state.world.player.position();
            util::make_explosion(&mut state.world.particles, &ppos, 8);

            state.reset();
        }
    }*/
}
