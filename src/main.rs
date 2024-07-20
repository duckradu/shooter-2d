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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (setup_camera, spawn_player))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_WIDTH as u32, TILE_HEIGHT as u32),
        SPRITE_SHEET_WIDTH as u32,
        SPRITE_SHEET_HEIGTH as u32,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn({
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        }
    });
}
