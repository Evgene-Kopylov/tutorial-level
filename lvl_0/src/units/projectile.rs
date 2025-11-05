use crate::settings::*;
use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::prelude::*;

/// Проектайл (снаряд), выпущенный игроком.
pub struct Projectile {
    pub texture: Texture2D,
    pub rotation: f32,
    pub start_position: Vec2,
    pub position: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub alive: bool,
}

impl Projectile {
    /// Создает новый экземпляр Projectile.
    ///
    /// ### Аргументы
    ///
    /// * `texture` - текстура проектайла.
    /// * `shoot_sound` - звук выстрела.
    /// * `rotation` - угол поворота проектайла.
    /// * `position` - начальная позиция проектайла.
    /// * `speed` - скорость проектайла.
    ///
    /// ### Возвращаемое значение
    ///
    /// Возвращает новый экземпляр структуры Projectile.
    pub fn new(
        texture: Texture2D,
        shoot_sound: Sound,
        rotation: f32,
        position: Vec2,
        speed: f32,
    ) -> Self {
        audio::play_sound(
            &shoot_sound,
            PlaySoundParams {
                volume: MAIN_UNIT_SHOOT_SOUND_VOLUME,
                looped: false
            },
        );

        let size = Vec2::new(texture.width(), texture.height());
        Self {
            texture,
            rotation,
            start_position: position,
            position,
            size,
            speed,
            alive: true,
        }
    }

    /// Обновляет позицию проектайла.
    ///
    /// ### Аргументы
    ///
    /// * `dt` - шаг времени.
    pub fn update_position(&mut self, dt: f32) {
        self.position.x += dt * self.speed * (self.rotation - f32::to_radians(90.)).cos();
        self.position.y += dt * self.speed * (self.rotation - f32::to_radians(90.)).sin();
    }

    /// Проверяет, находится ли проектайл в пределах экрана.
    pub fn is_within_screen_bounds(&self) -> bool {
        let half_width = self.size.x * 0.5;
        let half_height = self.size.y * 0.5;

        self.position.x + half_width >= 0.
            && self.position.x - half_width <= screen_width()
            && self.position.y + half_height >= 0.
            && self.position.y - half_height <= screen_height()
    }

    /// Обновляет состояние проектайла.
    ///
    /// ### Аргументы
    ///
    /// * `dt` - шаг времени.
    pub fn update(&mut self, dt: f32) {
        self.update_position(dt);
        self.alive = self.alive && self.is_within_screen_bounds();
    }

    /// Отрисовывает проектайл.
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - self.size.x * 0.50,
            self.position.y - self.size.y * 0.50,
            PROJECTILE_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}
