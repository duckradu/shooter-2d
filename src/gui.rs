use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    enemy::Enemy,
    player::{Health, Player},
    state::GameState,
    world::GameEntity,
};

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnExit(GameState::Menu), despawn_menu)
            .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::Bootstraping), spawn_debug_text)
            .add_systems(
                Update,
                update_debug_text.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct MenuItem;

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        })
        .insert(MenuItem);
}

fn despawn_menu(mut commands: Commands, menu_item_query: Query<Entity, With<MenuItem>>) {
    for e in menu_item_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn handle_menu_input(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => next_state.set(GameState::Bootstraping),
            _ => {}
        }
    }
}

#[derive(Component)]
struct DebugText;

fn spawn_debug_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Hello\nworld",
            TextStyle {
                font_size: 50.0,
                color: Color::BLACK,
                ..default()
            },
        ),
        DebugText,
        GameEntity,
    ));
}

fn update_debug_text(
    mut debug_text_query: Query<&mut Text, With<DebugText>>,
    diagnostics: Res<DiagnosticsStore>,
    player_query: Query<&Health, With<Player>>,
    enemy_query: Query<(), With<Enemy>>,
) {
    if debug_text_query.is_empty() || player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_health = player_query.single().0;
    let num_enemies = enemy_query.iter().count();

    let mut text = debug_text_query.single_mut();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("{value:.2}\n{num_enemies}\n{player_health}")
        }
    }
}
