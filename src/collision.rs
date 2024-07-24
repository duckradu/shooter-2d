use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::{constants::*, enemy::Enemy, state::GameState, weapon::Projectile};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyKdTree::default()).add_systems(
            Update,
            (
                handle_projectile_enemy_collision,
                update_enemy_kd_tree.run_if(on_timer(Duration::from_secs_f32(KD_TREE_UPDATE_RATE))),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Collidable {
    position: Vec2,
    entity: Entity,
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = typenum::U2;

    fn at(&self, i: usize) -> Self::Scalar {
        if i == 0 {
            return self.position.x;
        }

        self.position.y
    }
}

#[derive(Resource)]
struct EnemyKdTree(KdTree<Collidable>);

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();

    for (t, e) in enemy_query.iter() {
        items.push(Collidable {
            position: t.translation.truncate(),
            entity: e,
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

fn handle_projectile_enemy_collision(
    projectile_query: Query<&Transform, With<Projectile>>,
    tree: Res<EnemyKdTree>,
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
) {
    if projectile_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for projectile_transform in projectile_query.iter() {
        let position = projectile_transform.translation;

        let enemies = tree.0.within_radius(&[position.x, position.y], 50.0);

        for e in enemies {
            if let Ok(mut enemy) = enemy_query.get_mut(e.entity) {
                enemy.health -= PROJECTILE_DAMAGE;
            }
        }
    }
}
