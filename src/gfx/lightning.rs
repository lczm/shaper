use crate::constants::{LIGHTNING_BLOOM_COLOR, LIGHTNING_CORE_COLOR};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct LightningEffect {
    pub start: Vec2,
    pub end: Vec2,
    pub elapsed: f32,
    pub max_duration: f32,
}

impl LightningEffect {
    pub fn draw(&self) {
        let start_point = self.start;
        let end_point = self.end;

        if start_point.distance_squared(end_point) < 0.01 {
            return;
        }

        let mid_point = (start_point + end_point) * 0.5;
        let direction = end_point - start_point;
        let perpendicular_normal = vec2(-direction.y, direction.x).normalize_or_zero();

        let offset_distance = direction.length() * 0.2;
        let control_point = mid_point + perpendicular_normal * offset_distance;

        let alpha = 1.0 - (self.elapsed / self.max_duration);
        let bloom_color = Color {
            a: LIGHTNING_BLOOM_COLOR.a * 0.45 * alpha,
            ..LIGHTNING_BLOOM_COLOR
        };
        let core_color = Color {
            a: LIGHTNING_CORE_COLOR.a * alpha,
            ..LIGHTNING_CORE_COLOR
        };

        let segments = 8;
        let mut previous_point = start_point;

        // run through the segments like a bezier curve, drawing lines between each segment point
        for i in 1..=segments {
            let progress = i as f32 / segments as f32;
            let inverse_progress = 1.0 - progress;
            let segment_point = inverse_progress * inverse_progress * start_point
                + 2.0 * inverse_progress * progress * control_point
                + progress * progress * end_point;

            draw_line(
                previous_point.x,
                previous_point.y,
                segment_point.x,
                segment_point.y,
                4.0,
                bloom_color,
            );
            draw_line(
                previous_point.x,
                previous_point.y,
                segment_point.x,
                segment_point.y,
                1.5,
                core_color,
            );

            previous_point = segment_point;
        }
    }
}
