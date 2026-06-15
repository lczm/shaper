pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 800.0;

// gap from left window edge to arena border
pub const ARENA_MARGIN_WIDTH: f32 = 80.0;
// gap from top/bottom window edge to arena border
pub const ARENA_MARGIN_HEIGHT: f32 = 40.0;
pub const ARENA_BORDER_THICKNESS: f32 = 2.0;

pub const PLAYER_CIRCLE_RADIUS: f32 = 15.0;
pub const PLAYER_SPEED: f32 = 450.0;

pub const BOSS_WIDTH: f32 = 150.0;
pub const BOSS_HEIGHT: f32 = 150.0;

// clockwise spin speed in radians per second
// used when boss is idle
pub const BOSS_IDLE_ROTATION_SPEED: f32 = 0.8;

pub const PHASE_DISTANCE: f32 = 150.0;
pub const PHASE_DURATION: f32 = 0.20;
pub const PHASE_MIN_OPACITY: f32 = 0.1;
pub const PHASE_TRAIL_LENGTH: usize = 3; // ghost circles drawn behind the phase
pub const PHASE_GHOST_OPACITY: f32 = 0.2; // opacity of the freshest ghost

// initial game state
pub const STARTING_LIVES: u32 = 3;
