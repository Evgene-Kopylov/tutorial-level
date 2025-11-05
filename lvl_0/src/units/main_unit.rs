use crate::command::Command;
use crate::settings::*;
use macroquad::prelude::*;

/// Основной юнит, под контролем игрока.
pub struct MainUnit {
    pub texture: Texture2D,
    pub size: Vec2,
    pub _scale: f32,
    pub _radius: f32,
    pub rotation: f32,
    pub position: Vec2,
    pub speed: f32,
    pub shoot_timer: f32,
    shoot_delay: f32,
    pub shoot_range: f32,
    pub auto_aim: bool,
    bullet_load: u8,
}

impl MainUnit {
    /// Создает новый экземпляр MainUnit.
    ///
    /// ### Аргументы
    ///
    /// * `texture` - текстура для отрисовки юнита.
    /// * `position` - начальное положение юнита.
    ///
    /// ### Возвращаемое значение
    ///
    /// Возвращает новый экземпляр структуры MainUnit.
    pub fn new(texture: Texture2D, position: Vec2) -> Self {
        Self {
            texture: texture.clone(),
            position,
            size: Vec2::new(texture.width(), texture.height()),
            _scale: 1.,
            _radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            speed: MAIN_UNIT_SPEED,
            shoot_timer: 0.,
            shoot_delay: MAIN_UNIT_SHOOT_DELAY,
            shoot_range: MAIN_UNIT_SHOOT_RANGE,
            auto_aim: false,
            bullet_load: 0,
        }
    }

    /// Обновляет состояние юнита.
    ///
    /// ### Аргументы
    ///
    /// * `dt` - шаг времени.
    /// * `target_point` - позиция цели.
    /// * `command` - команда для управления юнитом.
    pub fn update(&mut self, dt: f32, target_point: Vec2, command: &mut Command) {
        self.shoot_timer += dt;
        self.update_position(dt, command);
        self.update_rotation(target_point, command);
        self.update_shooting(command);
    }

    /// Отрисовывает юнит.
    pub fn draw(&self) {
        self.draw_shadow();
        self.draw_main_unit();
    }

    /// Обновляет позицию юнита.
    fn update_position(&mut self, dt: f32, command: &Command) {
        self.position.x += command.wasd.x * dt * self.speed;
        self.position.y += command.wasd.y * dt * self.speed;

        if command.wasd.x != 0. || command.wasd.y != 0. || is_mouse_button_down(MouseButton::Left) {
            self.auto_aim = false;
        }
    }

    /// Обновляет угол поворота юнита.
    fn update_rotation(&mut self, target_point: Vec2, command: &Command) {
        self.rotation %= f32::to_radians(360.);
        let mut dx = self.position.x - target_point.x;
        if dx == 0f32 {
            dx += 1f32;
        };

        let mut dy = self.position.y - target_point.y;
        if dy == 0f32 {
            dy += 1f32;
        };

        if self.auto_aim {
            self.rotation = command.rotation;
        } else if !self.auto_aim {
            if dx >= 0f32 {
                self.rotation = (dy / dx).atan() - f32::to_radians(90.);
            } else {
                self.rotation = (dy / dx).atan() - f32::to_radians(270.);
            }
        }
    }

    /// Обновляет стрельбу юнита.
    fn update_shooting(&mut self, command: &mut Command) {
        if self.shoot_timer >= self.shoot_delay {
            if is_mouse_button_down(MouseButton::Left) {
                command.shoot = true;
                self.bullet_load = 0;
            } else if self.bullet_load > 0 {
                command.shoot = true;
                self.bullet_load -= 1;
            }
        } else {
            command.shoot = false;
        }

        if command.shoot {
            self.shoot_timer = 0.;
        }
    }

    /// Отрисовывает тень юнита.
    fn draw_shadow(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - self.size.x * 0.5 + 3.,
            self.position.y - self.size.y * 0.5 + 4.,
            DARKGRAY,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }

    /// Отрисовывает главный объект юнита.
    fn draw_main_unit(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - self.size.x * 0.5,
            self.position.y - self.size.y * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            },
        );
    }
}
