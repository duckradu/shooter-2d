// Window
pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 700.0;

pub const WINDOW_BG_COLOR: (u8, u8, u8) = (197, 204, 184);

// Spritesheet
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SHEET_WIDTH: usize = 8;
pub const SPRITE_SHEET_HEIGTH: usize = 8;

pub const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Tiles
pub const TILE_WIDTH: usize = 16;
pub const TILE_HEIGHT: usize = 16;

// World
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2500.0;
pub const NUM_WORLD_DECORATIONS: usize = 1000;

// Player
pub const PLAYER_SPEED: f32 = 2.0;

// Projectile
pub const PROJECTILE_SPAWN_INTERVAL: f32 = 0.1;
pub const PROJECTILE_SPEED: f32 = 15.0;
pub const PROJECTILE_DAMAGE: f32 = 50.0;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 500;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0;
pub const ENEMY_HEALTH: f32 = 100.0;
