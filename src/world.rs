use bevy::{math::vec3, prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{
    animation::AnimationTimer,
    constants::*,
    player::{Health, Player, PlayerState},
    state::GameState,
    weapon::{Weapon, WeaponTimer},
    GlobalTextureAtlas,
};

#[derive(Component)]
pub struct GameEntity;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Bootstraping),
            (init_world, decorate_world),
        )
        .add_systems(OnExit(GameState::Playing), despawn_game_entities);
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
        Health(PLAYER_HEALTH),
        GameEntity,
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
        GameEntity,
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
            GameEntity,
        ));
    }
}

fn despawn_game_entities(
    mut commands: Commands,
    game_entities_query: Query<Entity, With<GameEntity>>,
) {
    for e in game_entities_query.iter() {
        commands.entity(e).despawn_recursive()
    }
}
