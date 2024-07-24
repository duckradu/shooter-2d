use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{enemy::Enemy, state::GameState};

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Bootstraping), spawn_debug_text)
            .add_systems(
                Update,
                update_debug_text.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_debug_text(mut commands: Commands) {
    commands.spawn(TextBundle::from_section(
        "Hello\nworld",
        TextStyle {
            font_size: 50.0,
            color: Color::BLACK,
            ..default()
        },
    ));
}

fn update_debug_text(
    mut query: Query<&mut Text, With<Text>>,
    diagnostics: Res<DiagnosticsStore>,
    enemy_query: Query<(), With<Enemy>>,
) {
    if query.is_empty() {
        return;
    }

    let num_enemies = enemy_query.iter().count();
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("{value:.2}\n{num_enemies}")
        }
    }
}
