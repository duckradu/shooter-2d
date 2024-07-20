use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::vec3;
use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 700.0;

const WINDOW_BG_COLOR: (u8, u8, u8) = (197, 204, 184);

const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SHEET_WIDTH: usize = 4;
const SPRITE_SHEET_HEIGTH: usize = 4;

const SPRITE_SCALE_FACTOR: f32 = 3.0;

const TILE_WIDTH: usize = 16;
const TILE_HEIGHT: usize = 16;

// Player
const PLAYER_SPEED: f32 = 2.0;

// Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

// Components
#[derive(Component)]
struct Player;

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
        // Plugins
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        // Game state
        .init_state::<GameState>()
        // Systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::Bootstraping), (setup_camera, init_world))
        .add_systems(Update, (handle_input).run_if(in_state(GameState::Playing)))
        // .add_systems(Startup, (setup_camera, spawn_player))
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

    next_state.set(GameState::Playing);
}

fn handle_input(
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
