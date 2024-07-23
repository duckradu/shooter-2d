use bevy::prelude::*;

use crate::{
    constants::*,
    player::{Player, PlayerState},
    state::GameState,
    weapon::Weapon,
    CursorPosition,
};

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animation_timer_tick, animate_player, animate_weapon)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<
        (
            &mut TextureAtlas,
            &mut Sprite,
            &Transform,
            &PlayerState,
            &AnimationTimer,
        ),
        With<Player>,
    >,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut texture_atlas, mut sprite, transform, player_state, timer) = player_query.single_mut();

    if timer.just_finished() {
        let base_sprite_index = match player_state {
            PlayerState::Idle => 0,
            PlayerState::Moving => 4,
        };

        texture_atlas.index =
            base_sprite_index + (texture_atlas.index + 1) % (SPRITE_SHEET_WIDTH / 2);
    }

    if let Some(cursor_position) = cursor_position.0 {
        sprite.flip_x = cursor_position.x < transform.translation.x;
    }
}

fn animate_weapon(
    cursor_position: Res<CursorPosition>,
    mut weapon_query: Query<(&mut Sprite, &Transform), With<Weapon>>,
) {
    if weapon_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = weapon_query.single_mut();

    if let Some(cursor_position) = cursor_position.0 {
        sprite.flip_y = cursor_position.x < transform.translation.x;
    }
}
