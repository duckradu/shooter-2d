use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use shooter2d::camera::FollowCameraPlugin;
use shooter2d::player::PlayerPlugin;
use shooter2d::state::GameState;
use shooter2d::weapon::WeaponPlugin;
use shooter2d::world::WorldPlugin;
use shooter2d::{constants::*, ResourcesPlugin};

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
        // Development Plugins
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        // Game state
        .init_state::<GameState>()
        // Game plugins
        .add_plugins((
            ResourcesPlugin,
            FollowCameraPlugin,
            WorldPlugin,
            PlayerPlugin,
            WeaponPlugin,
        ))
        .run();
}
