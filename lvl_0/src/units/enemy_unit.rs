use crate::settings::*;
use crate::Vec2;
use macroquad::audio::Sound;
use macroquad::color::{BLACK, GREEN};
use macroquad::prelude::{draw_texture_ex, Color, DrawTextureParams, Texture2D, BROWN}; // FIXME

#[derive(Clone)]
pub struct EnemyUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    color: Color,
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
    pub shift: Vec2,
    pub impact_sound: Sound,
    pub(crate) hit_points: f32,
    pub(crate) alive: bool,
}

impl EnemyUnit {
    pub fn new(
        texture: Texture2D,
        shadow_texture: Texture2D,
        impact_sound: Sound,
        spawn_position: Vec2,
    ) -> Self {
        let color = BLACK;

        Self {
            texture: texture.clone(),
            shadow_texture,
            color,
            position: spawn_position,
            rotation: f32::to_radians(-90.0),
            radius: texture.width() * 0.5,
            shift: Vec2::new(0., 0.),
            impact_sound,
            hit_points: 100.,
            alive: true,
        }
    }

    pub fn draw(&self) {
        let color = if self.alive { BROWN } else { GREEN };

        draw_texture_ex(
            &self.texture,
            self.position.x - self.texture.width() * 0.5 + self.shift.x,
            self.position.y - self.texture.height() * 0.5 - self.shift.y,
            color,
            DrawTextureParams {
                rotation: self.rotation - f32::to_radians(90.),
                ..Default::default()
            },
        );
    }

    pub fn draw_shadow(&self) {
        // тень
        let height = 1.6;
        let mut color = self.color;
        color.a = 0.2;
        draw_texture_ex(
            &self.texture,
            self.position.x - self.texture.width() * 0.5 + 3. * height,
            self.position.y - self.texture.height() * 0.5 + 4. * height,
            color,
            DrawTextureParams {
                rotation: self.rotation - f32::to_radians(90.),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, dt: f32, target: Vec2, units: Vec<EnemyUnit>, exclude: usize) {
        self.rotation %= f32::to_radians(360.);
        let mut dx = self.position.x - target.x;
        if dx == 0f32 {
            dx += 1f32;
        };

        let mut dy = self.position.y - target.y;
        if dy == 0f32 {
            dy += 1f32;
        };

        // абсолютный угол к целиwww
        let a: f32 = if dx >= 0. {
            (dy / dx).atan()
        } else {
            (dy / dx).atan() - f32::to_radians(180.)
        };

        // относительный угол
        let mut da = self.rotation - a;

        // убрать намотку угла
        if da <= f32::to_radians(-180.) {
            da += f32::to_radians(360.)
        }
        if da > f32::to_radians(180.) {
            da -= f32::to_radians(360.)
        }

        // сохранение направления движения
        if da.abs() > f32::to_radians(9.) {
            if da > 0. {
                self.rotation -= dt * ENEMY_UNIT_ROTATION_SPEED
            } else {
                self.rotation += dt * ENEMY_UNIT_ROTATION_SPEED
            }
        }

        self.swarm_behaviour(dt, units, exclude);

        self.position.x += -1. * dt * ENEMY_UNIT_SPEED * self.rotation.cos();
        self.position.y += -1. * dt * ENEMY_UNIT_SPEED * self.rotation.sin();
    }

    /// Роевое поведение
    fn swarm_behaviour(&mut self, dt: f32, units: Vec<EnemyUnit>, exclude: usize) {
        // отворот от близкого юнита
        for (i, unit) in units.iter().enumerate() {
            if i == exclude {
                continue;
            }
            let x0 = self.position.x;
            let y0 = self.position.y;
            let x1 = unit.position.x;
            let y1 = unit.position.y;
            let dx = x0 - x1;
            let dy = y0 - y1;
            let distance = (dx.powf(2.) + dy.powf(2.)).sqrt();
            if distance < 70. {
                // соседний юнит близко

                // абсолютный угол
                let a: f32 = if dx >= 0. {
                    (dy / dx).atan()
                } else {
                    (dy / dx).atan() - f32::to_radians(180.)
                };

                // относительный угол
                let da = self.rotation - a;

                // отворачивать от близкого юнита
                if da < 0. && da > -20. {
                    self.rotation -= 0.7 * dt * ENEMY_UNIT_ROTATION_SPEED
                } else if da > 0. && da < 20. {
                    self.rotation += 0.7 * dt * ENEMY_UNIT_ROTATION_SPEED
                }
            }
        }
    }
}
