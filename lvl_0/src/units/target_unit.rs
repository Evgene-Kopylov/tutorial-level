use crate::{Vec2, TARGET_UNIT_IMPACT_SOUND_VOLUME};
use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::color::{BLACK, GREEN, WHITE};
use macroquad::prelude::{draw_texture_ex, Color, DrawTextureParams, Texture2D};

pub struct TargetUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    color: Color,
    pub position: Vec2,
    pub radius: f32,
    pub shift: Vec2,
    impact_sound: Sound,
    pub(crate) hit_points: f32,
    pub alive: bool,
}

impl TargetUnit {
    pub fn new(
        texture: Texture2D,
        shadow_texture: Texture2D,
        impact_sound: Sound,
        spawn_position: Vec2,
    ) -> Self {
        let mut color = BLACK;
        color.a = 0.45;

        Self {
            texture,
            shadow_texture,
            color,
            position: spawn_position,
            radius: texture.width() * 0.5,
            shift: Vec2::new(0., 0.),
            impact_sound,
            hit_points: 100.,
            alive: true,
        }
    }

    pub fn update_movement(&mut self, impact: bool, impact_angle: f32) {
        if impact {
            self.hit_points += impact_angle.cos() * 5.0;

            if self.hit_points <= 0. {
                self.alive = false;
            }

            let shift = 5.;
            self.shift = Vec2::new(shift * impact_angle.sin(), shift * impact_angle.cos());

            let volume = if self.alive {
                TARGET_UNIT_IMPACT_SOUND_VOLUME
            } else {
                TARGET_UNIT_IMPACT_SOUND_VOLUME * 0.25
            };

            audio::play_sound(
                self.impact_sound,
                PlaySoundParams {
                    volume,
                    ..Default::default()
                },
            );
        }
    }

    pub fn update(&mut self, impact: bool, hit_points: f32, impact_angle: f32) {
        self.hit_points += hit_points;
        self.update_movement(impact, impact_angle);
    }

    pub fn draw(&self) {
        let color = if self.alive { WHITE } else { GREEN };

        draw_texture_ex(
            self.texture,
            self.position.x - self.texture.width() * 0.5 + self.shift.x,
            self.position.y - self.texture.height() * 0.5 - self.shift.y,
            color,
            DrawTextureParams {
                ..Default::default()
            },
        );
    }

    pub fn draw_shadow(&self) {
        // тень
        let height = 3.;
        draw_texture_ex(
            self.shadow_texture,
            self.position.x - self.texture.width() * 0.5 + 3. * height,
            self.position.y - self.texture.height() * 0.5 + 4. * height,
            self.color,
            DrawTextureParams {
                ..Default::default()
            },
        );
    }
}
