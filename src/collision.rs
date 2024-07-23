use bevy::prelude::*;

use crate::{enemy::Enemy, state::GameState, weapon::Projectile, PROJECTILE_DAMAGE};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_projectile_enemy_collision.run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_projectile_enemy_collision(
    projectile_query: Query<&Transform, With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if projectile_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for projectile_transform in projectile_query.iter() {
        for (enemy_transform, mut e) in enemy_query.iter_mut() {
            if projectile_transform
                .translation
                .distance_squared(enemy_transform.translation)
                <= 1000.0
            {
                e.health -= PROJECTILE_DAMAGE;
            }
        }
    }
}
