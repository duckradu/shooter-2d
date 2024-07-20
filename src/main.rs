use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
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
}
