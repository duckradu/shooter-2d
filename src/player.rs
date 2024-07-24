use bevy::{math::vec3, prelude::*};

use crate::{constants::*, state::GameState};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
            Update,
            (
                handle_player_input,
                handle_player_enemy_collision_events,
                handle_player_death,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state) = player_query.single_mut();

    let up_key: bool =
        keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let right_key: bool =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    let down_key: bool =
        keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let left_key: bool =
        keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);

    let mut delta = Vec2::ZERO;

    if up_key {
        delta.y += 1.0;
    }
    if down_key {
        delta.y -= 1.0;
    }
    if left_key {
        delta.x -= 1.0;
    }
    if right_key {
        delta.x += 1.0;
    }

    delta = delta.normalize();

    if delta.is_finite() && (up_key || right_key || down_key || left_key) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        transform.translation.z = 10.0;

        *player_state = PlayerState::Moving;
    } else {
        *player_state = PlayerState::Idle;
    }
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

fn handle_player_enemy_collision_events(
    mut player_query: Query<&mut Health, With<Player>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut health = player_query.single_mut();

    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE
    }
}

fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }

    let health = player_query.single();

    if health.0 <= 0.0 {
        next_state.set(GameState::Menu)
    }
}
