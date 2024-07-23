use bevy::{math::vec3, prelude::*};

use crate::{constants::*, state::GameState};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::Playing)),
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
