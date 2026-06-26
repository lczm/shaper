use macroquad::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 800.0;

// color scheme
pub const BACKGROUND: Color = Color::new(0.06, 0.07, 0.11, 1.0); // near-black navy
pub const ARENA_BORDER_COLOR: Color = Color::new(0.42, 0.44, 0.72, 1.0); // soft indigo frame

// player colors
pub const PLAYER_COLOR: Color = Color::new(0.25, 0.92, 0.85, 1.0); // cyan
pub const PLAYER_PHASING_COLOR: Color = Color::new(0.85, 1.0, 1.0, 1.0); // pale ghost cyan
pub const PLAYER_PROJECTILE_COLOR: Color = Color::new(0.55, 0.95, 1.0, 1.0); // light cyan
pub const PLAYER_TRAIL_COLOR: Color = Color::new(0.45, 0.85, 0.85, 1.0); // dim cyan

// boss colors
pub const BOSS_COLOR: Color = Color::new(0.96, 0.28, 0.55, 1.0); // hot magenta-pink
pub const BOSS_PROJECTILE_COLOR: Color = Color::new(1.0, 0.45, 0.30, 1.0); // warm coral

// hud colors
pub const UI_TEXT_COLOR: Color = Color::new(0.90, 0.92, 0.96, 1.0); // soft off-white

// gap from left window edge to arena border
pub const ARENA_MARGIN_WIDTH: f32 = 80.0;
// gap from top/bottom window edge to arena border
pub const ARENA_MARGIN_HEIGHT: f32 = 40.0;
pub const ARENA_BORDER_THICKNESS: f32 = 2.0;

pub const PLAYER_CIRCLE_RADIUS: f32 = 10.0;
pub const PLAYER_SPEED: f32 = 450.0;
// seconds between player shots (the player fires continuously)
pub const PLAYER_FIRE_INTERVAL: f32 = 0.12;
// upward travel speed of player bullets in pixels per second
pub const PLAYER_PROJECTILE_SPEED: f32 = 600.0;

pub const BOSS_WIDTH: f32 = 150.0;
pub const BOSS_HEIGHT: f32 = 150.0;

// clockwise spin speed in radians per second
// used when boss is idle (the steady speed it settles to)
pub const BOSS_IDLE_ROTATION_SPEED: f32 = 0.8;
// fastest speed
pub const BOSS_SPINUP_PEAK_SPEED: f32 = 12.0;
// slow -> fast for this many seconds
pub const BOSS_SPINUP_RAMP_UP: f32 = 0.8;
// hold atp eak for how long
pub const BOSS_SPINUP_HOLD: f32 = 1.0;
// easing down for this many seconds
pub const BOSS_SPINUP_RAMP_DOWN: f32 = 1.5;
// total duration
pub const BOSS_SPINUP_DURATION: f32 =
    BOSS_SPINUP_RAMP_UP + BOSS_SPINUP_HOLD + BOSS_SPINUP_RAMP_DOWN;

// seconds between shots while the boss is idle
pub const BOSS_FIRE_INTERVAL: f32 = 0.5;
// number of projectiles in each all-directions burst
pub const BOSS_PROJECTILE_COUNT: usize = 15;

pub const PROJECTILE_RADIUS: f32 = 6.0;
// travel speed in pixels per second
pub const PROJECTILE_SPEED: f32 = 120.0;

// persistent sweeping beam fired by the boss
pub const BEAM_WIDTH: f32 = 16.0;
// beam colour when active
pub const BEAM_COLOR: Color = Color::new(1.0, 0.25, 0.35, 0.95);
// the thickness to show when the beam is inactive
pub const BEAM_BORDER_THICKNESS: f32 = 2.0;
// beam time taken to become active
pub const BEAM_STARTUP_DURATION: f32 = 1.5;
pub const BEAM_ACTIVE_DURATION: f32 = 3.0;

// invulnerability window granted to the player after any hit
pub const HIT_INVULN_DURATION: f32 = 0.5;

// screen shake (trauma-based): each hit bumps trauma, which decays every second.
// the visible shake scales with trauma^2 so small bumps stay subtle.
pub const SHAKE_TRAUMA_PER_HIT: f32 = 0.6;
// trauma lost per second; 0.6 trauma fully decays in 0.6 / 1.2 = 0.5s
pub const SHAKE_DECAY: f32 = 1.2;
// peak positional offset in logical units at full trauma
pub const SHAKE_MAX_OFFSET: f32 = 16.0;
// peak rotational kick in degrees at full trauma (set to 0.0 to disable tilt)
pub const SHAKE_MAX_ANGLE: f32 = 2.5;
// how fast the shake noise oscillates, in noise samples per second.
pub const SHAKE_FREQUENCY: f32 = 15.0;
pub const SHAKE_MIN_TRAUMA: f32 = 0.01;

pub const PHASE_DISTANCE: f32 = 150.0;
pub const PHASE_DURATION: f32 = 0.20;
pub const PHASE_MIN_OPACITY: f32 = 0.1;
pub const PHASE_TRAIL_LENGTH: usize = 3; // ghost circles drawn behind the phase
pub const PHASE_GHOST_OPACITY: f32 = 0.2; // opacity of the freshest ghost

// initial game state
pub const STARTING_LIVES: u32 = 3;
pub const STARTING_BOMBS: u32 = 3;

// boss health bar (top-of-screen hud)
pub const HEALTH_BAR_HEIGHT: f32 = 28.0;
// gap from the top window edge down to the bar
pub const HEALTH_BAR_TOP_MARGIN: f32 = 8.0;
// empty track colour
pub const HEALTH_BAR_BG_COLOR: Color = Color::new(0.15, 0.16, 0.22, 0.9);
// current health colour
pub const HEALTH_BAR_FILL_COLOR: Color = Color::new(0.90, 0.20, 0.25, 1.0);
