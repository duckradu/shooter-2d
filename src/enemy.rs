use std::time::Duration;

use bevy::{math::vec3, prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::{animation::AnimationTimer, player::Player, state::GameState, *};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemy_wave.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_enemy_wave(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(10);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..enemy_spawn_count {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 12,
            },
            Enemy::default(),
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
        ));
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation;

    for mut transform in enemy_query.iter_mut() {
        let dir = (player_position - transform.translation).normalize();

        transform.translation += dir * ENEMY_SPEED;
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn()
        }
    }
}
