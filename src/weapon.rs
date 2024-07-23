use std::{f32::consts::PI, time::Instant};

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::Stopwatch,
};
use rand::Rng;

use crate::{player::Player, state::GameState, *};

pub struct WeaponPlugin;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct WeaponTimer(pub Stopwatch);

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct SpawnInstant(Instant);

#[derive(Component)]
struct ProjectileDirection(Vec3);

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_weapon_transform,
                handle_weapon_input,
                update_projectile,
                despawn_old_projectiles,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_weapon_transform(
    player_query: Query<&Transform, With<Player>>,
    mut weapon_query: Query<&mut Transform, (With<Weapon>, Without<Player>)>,
    cursor_position: Res<CursorPosition>,
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation.truncate();
    let cursor_position = match cursor_position.0 {
        Some(pos) => pos,
        None => player_position,
    };
    let mut weapon_transform = weapon_query.single_mut();

    let angle =
        (player_position.y - cursor_position.y).atan2(player_position.x - cursor_position.x) + PI;

    weapon_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_weapon_position = vec2(
        player_position.x + offset * angle.cos() - 5.0,
        player_position.y + offset * angle.sin() - 15.0,
    );

    weapon_transform.translation = vec3(new_weapon_position.x, new_weapon_position.y, 15.0);
}

fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>,
    mut weapon_query: Query<(&Transform, &mut WeaponTimer), With<Weapon>>,
) {
    if weapon_query.is_empty() {
        return;
    }

    let (weapon_transform, mut weapon_timer) = weapon_query.single_mut();
    let weapon_position = weapon_transform.translation.truncate();

    weapon_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    if weapon_timer.0.elapsed_secs() >= PROJECTILE_SPAWN_INTERVAL {
        weapon_timer.0.reset();

        let mut rng = rand::thread_rng();
        let projectile_direction = weapon_transform.local_x();

        for _ in 0..3 {
            let direction = vec3(
                projectile_direction.x + rng.gen_range(-0.5..0.5),
                projectile_direction.y + rng.gen_range(-0.5..0.5),
                projectile_direction.z,
            );

            commands.spawn((
                SpriteBundle {
                    texture: handle.image.clone().unwrap(),
                    transform: Transform::from_translation(vec3(
                        weapon_position.x,
                        weapon_position.y,
                        1.0,
                    ))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 16,
                },
                SpawnInstant(Instant::now()),
                Projectile,
                ProjectileDirection(direction),
            ));
        }
    }
}

fn update_projectile(
    mut projectile_query: Query<(&mut Transform, &ProjectileDirection), With<Projectile>>,
) {
    if projectile_query.is_empty() {
        return;
    }

    for (mut t, dir) in projectile_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(PROJECTILE_SPEED);
        t.translation.z = 10.0;
    }
}

fn despawn_old_projectiles(
    mut commands: Commands,
    projectile_query: Query<(&SpawnInstant, Entity), With<Projectile>>,
) {
    for (instant, entity) in projectile_query.iter() {
        if instant.0.elapsed().as_secs_f32() > 0.3 {
            commands.entity(entity).despawn();
        }
    }
}
