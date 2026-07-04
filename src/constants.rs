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
pub const PLAYER_SPEED: f32 = 300.0;
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
// the fan's aim ping-pongs left/right, one step per volley. BOSS_AIM_STEP is the
// per-volley angle change (~7 degrees); BOSS_AIM_STEPS is how many steps it takes
// to reach each extreme. so the sweep spans +/- (STEP * STEPS) radians and, because
// the turn lands exactly on a step boundary, reverses seamlessly.
pub const BOSS_AIM_STEP: f32 = 0.12;
pub const BOSS_AIM_STEPS: i32 = 3;

pub const PROJECTILE_RADIUS: f32 = 6.0;
// travel speed in pixels per second
pub const PROJECTILE_SPEED: f32 = 120.0;

// the boss fires a beam volley on this interval. the telegraph + active window is
// ~4.5s (see BEAM_STARTUP_DURATION + BEAM_ACTIVE_DURATION), so this stays comfortably
// longer to keep the beam attack infrequent
pub const BOSS_BEAM_INTERVAL: f32 = 6.0;

// persistent sweeping beam fired by the boss
pub const BEAM_WIDTH: f32 = 16.0;
// beams are drawn overshooting the arena by this much so their flat end caps never
// leave a triangular gap at the edges; the overflow is hidden by the arena frame mask
pub const BEAM_EDGE_OVERSHOOT: f32 = 50.0;
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
// animated chip colour
pub const HEALTH_BAR_CHIP_COLOR: Color = Color::new(0.98, 0.90, 0.65, 1.0);
pub const HEALTH_BAR_DROP_SPEED: f32 = 9.0;

// bomb: clears nearby enemy hazards for a short window when detonated
// clear radius around the player when the bomb goes off
pub const BOMB_RADIUS: f32 = 200.0;
// how long the bomb stays active (clearing + visible), in seconds.
// time-based so it's framerate independent (~10 frames at 60fps)
pub const BOMB_DURATION: f32 = 0.16;
// the ring drawn while the bomb is active
pub const BOMB_BORDER_COLOR: Color = Color::new(0.85, 0.95, 1.0, 1.0); // bright pale cyan
pub const BOMB_BORDER_THICKNESS: f32 = 3.0;

// transient "Reset" banner shown centered on screen after an admin reset
pub const RESET_BANNER_DURATION: f32 = 0.75;
pub const RESET_BANNER_FONT_SIZE: f32 = 96.0;

// lost banner
pub const LOST_BANNER_DURATION: f32 = 0.75;
pub const LOST_BANNER_FONT_SIZE: f32 = 96.0;
