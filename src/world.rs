use bevy::{math::vec3, prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{
    animation::AnimationTimer,
    constants::*,
    player::{Player, PlayerState},
    state::GameState,
    weapon::{Weapon, WeaponTimer},
    GlobalTextureAtlas,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Bootstraping),
            (init_world, decorate_world),
        );
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 0,
        },
        Player,
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 17,
        },
        Weapon,
        WeaponTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::Playing);
}

fn decorate_world(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=25),
            },
        ));
    }
}
