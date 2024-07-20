use std::f32::consts::PI;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::PrimaryWindow;
use rand::Rng;

// Window
const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 700.0;

const WINDOW_BG_COLOR: (u8, u8, u8) = (197, 204, 184);

// Spritesheet
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SHEET_WIDTH: usize = 8;
const SPRITE_SHEET_HEIGTH: usize = 8;

const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Tiles
const TILE_WIDTH: usize = 16;
const TILE_HEIGHT: usize = 16;

// World
const WORLD_W: f32 = 3000.0;
const WORLD_H: f32 = 2500.0;
const NUM_WORLD_DECORATIONS: usize = 1000;

// Player
const PLAYER_SPEED: f32 = 2.0;

// Projectile
const PROJECTILE_SPAWN_INTERVAL: f32 = 0.1;
const PROJECTILE_SPEED: f32 = 15.0;

// Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Resource)]
struct CursorPosition(Option<Vec2>);

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Weapon;

#[derive(Component)]
struct WeaponTimer(Stopwatch);

#[derive(Component)]
struct Projectile;

#[derive(Component)]
struct ProjectileDirection(Vec3);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Bootstraping,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb_u8(
            WINDOW_BG_COLOR.0,
            WINDOW_BG_COLOR.1,
            WINDOW_BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        // Custom resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPosition(None))
        // Plugins
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        // Game state
        .init_state::<GameState>()
        // Systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            OnEnter(GameState::Bootstraping),
            (setup_camera, init_world, decorate_world),
        )
        .add_systems(
            Update,
            (
                update_camera_position,
                update_cursor_position,
                update_weapon_transform,
                update_projectile,
                handle_player_input,
                handle_weapon_input,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));

    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH as u32, TILE_HEIGHT as u32),
        SPRITE_SHEET_WIDTH as u32,
        SPRITE_SHEET_HEIGTH as u32,
        None,
        None,
    );

    texture_atlas.0 = Some(texture_atlas_layouts.add(texture_atlas_layout));

    next_state.set(GameState::Bootstraping)
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_handle.0.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 0,
        },
        Player,
    ));
    commands.spawn((
        SpriteBundle {
            texture: image_handle.0.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 17,
        },
        Weapon,
        WeaponTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::Playing);
}

fn decorate_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        commands.spawn((
            SpriteBundle {
                texture: image_handle.0.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: rng.gen_range(24..=25),
            },
        ));
    }
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();

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
    }
}

fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
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

        let projectile_direction = weapon_transform.local_x();

        commands.spawn((
            SpriteBundle {
                texture: image_handle.0.clone().unwrap(),
                transform: Transform::from_translation(vec3(
                    weapon_position.x,
                    weapon_position.y,
                    1.0,
                ))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 16,
            },
            Projectile,
            ProjectileDirection(*projectile_direction),
        ));
    }
}

fn update_camera_position(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;

    camera_transform.translation = camera_transform
        .translation
        .lerp(vec3(player_transform.x, player_transform.y, 0.0), 0.1);
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_position.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    cursor_position.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
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

    weapon_transform.translation = vec3(
        new_weapon_position.x,
        new_weapon_position.y,
        weapon_transform.translation.z,
    );
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
