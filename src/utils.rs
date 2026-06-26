// smoothstep easing: maps u in [0, 1] onto an S-curve in [0, 1] that has zero
// slope at both ends, so interpolating with it eases in and out instead of
// moving at a constant rate. https://en.wikipedia.org/wiki/Smoothstep
pub fn smoothstep(u: f32) -> f32 {
    let u = u.clamp(0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}

// 1-D value noise in [-1, 1].
pub fn noise(seed: u32, t: f32) -> f32 {
    let i = t.floor();
    let f = t - i;
    let a = hash(seed, i as i32);
    let b = hash(seed, i as i32 + 1);
    let u = smoothstep(f);
    // lerp a..b by u, then remap [0, 1] -> [-1, 1]
    (a + (b - a) * u) * 2.0 - 1.0
}

// pseudo random f32 in [0, 1)]
pub fn hash(seed: u32, x: i32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x27d4_eb2d) ^ seed.wrapping_mul(0x9e37_79b9);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    (h & 0x00ff_ffff) as f32 / 0x0100_0000 as f32
}
