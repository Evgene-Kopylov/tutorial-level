use crate::assets::Assets;
use crate::command::Command;
use crate::settings::ENEMY_UNIT_IMPACT_SOUND_VOLUME;
use crate::units::enemy_unit::EnemyUnit;
use crate::units::main_unit::MainUnit;
use crate::units::projectile::Projectile;
use crate::units::target_unit::TargetUnit;
use crate::utils::get_command_line_argument;
use macroquad::audio::{self, PlaySoundParams};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{info, mouse_position, screen_height, screen_width, Vec2};
use macroquad::time::get_frame_time;
use quad_url::set_program_parameter;

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
    enemy_units: Vec<EnemyUnit>,
    projectiles: Vec<Projectile>,
    dt: f32,
    assets: Assets,
    command: Command,
    tick: f32,
    target_point: Vec2,
}

impl Scene {
    /// создание экземпляра Сцены
    pub async fn new() -> Self {
        let spawn_position = Vec2::new(screen_width() * 0.5, screen_height() * 0.8);
        let target_unit_position = Vec2::new(screen_width() * 0.5, 160.);

        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        let assets = Assets::new().await.unwrap();

        let mut scene = Self {
            main_unit: MainUnit::new(assets.main_unit_texture.clone(), spawn_position),
            target_unit: TargetUnit::new(
                assets.target_unit_texture.clone(),
                assets.target_unit_shadow_texture.clone(),
                assets.target_impact_sound.clone(),
                target_unit_position,
            ),
            enemy_units: Vec::new(),
            projectiles: vec![],
            dt,
            assets,
            command: Command::new(),
            tick: 1000., // большое число, чтобы сразу срабатывало
            target_point: mouse_position,
        };
        scene.start();
        scene
    }

    /// запустить игру.
    fn start(&mut self) {
        // спавн `enemy_units`
        // слево
        self.spawn_single_enemy_unit(-100., 0.);
        self.spawn_single_enemy_unit(-200., 0.);
        // справа
        self.spawn_single_enemy_unit(100., 0.);
        self.spawn_single_enemy_unit(200., 0.);
        // впереди
        // self.spawn_single_enemy_unit(0., 100.);
    }

    /// перезапустить игру
    /// здоровье юнитов и позиции будут восстановленны
    fn restart(&mut self) {
        // очистить поле
        self.enemy_units = vec![];

        // восстановить `target_unit`
        self.target_unit.hit_points = 100.;
        self.target_unit.alive = true;

        self.start();
    }

    /// создать enemy_unit по координатам относительно `target_unit`
    fn spawn_single_enemy_unit(&mut self, dx: f32, dy: f32) {
        let x = self.target_unit.position.x + dx;
        let y = self.target_unit.position.y + dy;
        let unit = EnemyUnit::new(
            self.assets.enemy_unit_gray.clone(),
            self.assets.target_unit_shadow_texture.clone(),
            self.assets.target_impact_sound.clone(),
            Vec2 { x, y },
        );
        self.enemy_units.push(unit);
    }

    /// Поймать активность пользователя.
    fn update_command_from_user_input(&mut self) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x_move += 1f32;
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y_move += 1f32;
        }

        if self.main_unit.position.x < 1f32 {
            x_move = 1f32;
        }
        if self.main_unit.position.x > screen_width() {
            x_move = -1f32;
        }

        if self.main_unit.position.y < 1f32 {
            y_move = 1f32;
        }
        if self.main_unit.position.y > screen_height() {
            y_move = -1f32;
        }
        self.command.wasd = Vec2::new(x_move, y_move);
    }

    /// Обновить `Command` из URL аргументов
    fn update_command_from_url_query(&mut self) {
        if get_command_line_argument("command") == *"Shoot" {
            // self.restart();
            self.command.shoot = true;
            let x = get_command_line_argument("target_point_x").parse().unwrap_or(0.);
            let y = get_command_line_argument("target_point_y").parse().unwrap_or(0.);
            self.target_point = Vec2::new(x, y);
            info!("{:?}", self.target_point);
            set_program_parameter("command", "");
            self.main_unit.shoot_timer = 1.; // чтобы получить выстрел с минимальной задержкой
            self.main_unit.auto_aim = true;
        }

        match get_command_line_argument("rotation").parse::<f32>() {
            Ok(a) => {
                self.command.rotation = a.to_radians();
            }
            Err(_e) => {
                // info!("{}", _e);
            }
        }
    }

    /// передать параметры в URL аргементы
    fn set_parameters_to_url_query(&mut self) {
        let line = format!(
            "({}, {})",
            self.target_unit.position.x as i32, self.target_unit.position.y as i32
        );
        set_program_parameter("target_pos", line.as_str());
        let line = format!(
            "({}, {})",
            self.main_unit.position.x as i32, self.main_unit.position.y as i32
        );
        set_program_parameter("unit_pos", line.as_str());

        let mut line = "[".to_string();
        for i in 0..self.enemy_units.len() {
            let value = format!(
                "({}, {}), ",
                self.enemy_units[i].position.x as i32, self.enemy_units[i].position.y as i32
            );
            line += &value;
        }
        line += "]";
        set_program_parameter("enemy_units", &line);
    }

    /// Обновить сцену
    pub fn update(&mut self) {
        self.tick += self.dt;
        self.update_command_from_user_input();

        if self.tick >= 1. {
            self.tick = 0.0;
            self.set_parameters_to_url_query();
            self.update_command_from_url_query();
        }
        self.dt = get_frame_time();
        self.target_unit.shift = Vec2::new(0., 0.);

        // стрельба и спавн выстрела
        self.main_unit_shoot();

        // удалить дохлые юниты
        self.remove_dead_enemy_units();

        // обновить всех коричневыз
        self.update_enemy_units();

        // Удаление снарядов на отлете
        self.remove_projectile_out_of_range();

        // поражение главной мишени
        self.target_unit_hit();

        // поражение enemy_units
        self.enemy_units_hit();
    }

    /// стрельба и спавн выстрела
    fn main_unit_shoot(&mut self) {
        let target_point = if self.target_point.x != 0. || self.target_point.y != 0. {
            self.target_point
        } else {
            mouse_position().into()
        };

        self.main_unit
            .update(self.dt, target_point, &mut self.command);
        if self.command.shoot {
            let position = Vec2::new(
                // точка появления выстрела
                self.main_unit.position.x
                    + 65. * (self.main_unit.rotation - f32::to_radians(90.)).cos(),
                self.main_unit.position.y
                    + 65. * (self.main_unit.rotation - f32::to_radians(90.)).sin(),
            );

            let projectile = Projectile::new(
                self.assets.projectile_texture.clone(),
                self.assets.main_unit_shoot_sound.clone(),
                self.main_unit.rotation,
                position,
                self.main_unit.speed * 3.,
            );
            self.projectiles.push(projectile);
        }
    }

    /// Обновить все `enemy_units`
    fn update_enemy_units(&mut self) {
        for i in 0..self.enemy_units.len() {
            let units = self.enemy_units.clone();
            self.enemy_units[i].update(self.dt, self.main_unit.position, units, i);
        }
    }

    /// удалить дохлые юниты
    fn remove_dead_enemy_units(&mut self) {
        self.enemy_units.retain(|u| u.hit_points > 0.);
    }

    /// Удаление снарядов на отлете
    fn remove_projectile_out_of_range(&mut self) {
        self.projectiles.retain(|p| {
            ((p.start_position.x - p.position.x).powf(2f32)
                + (p.start_position.y - p.position.y).powf(2f32)
                < self.main_unit.shoot_range.powf(2f32))
                && p.alive
        });
    }

    /// поражение главной мишени
    fn target_unit_hit(&mut self) {
        for i in 0..self.projectiles.len() {
            let p = &mut self.projectiles[i];

            if (p.position.x - self.target_unit.position.x).powf(2f32)
                + (p.position.y - self.target_unit.position.y).powf(2f32)
                < self.target_unit.radius.powf(2f32)
            {
                p.alive = false;
                self.target_unit.update(true, -20., p.rotation);
                info!("target_unit.hit_points: {:?}", self.target_unit.hit_points);
                if self.target_unit.hit_points <= -100. {
                    self.restart();
                }
            } else {
                p.update(self.dt);
            }
        }
    }

    /// поражение enemy_units
    fn enemy_units_hit(&mut self) {
        for i in 0..self.projectiles.len() {
            let p = &mut self.projectiles[i];
            for j in 0..self.enemy_units.len() {
                let u = &mut self.enemy_units[j];
                let dx = p.position.x - u.position.x;
                let dy = p.position.y - u.position.y;
                let dist = (dx.powf(2.) + dy.powf(2.)).sqrt();
                if dist < u.radius {
                    u.hit_points -= 20.;
                    audio::play_sound(
                        &u.impact_sound,
                        PlaySoundParams {
                            volume: ENEMY_UNIT_IMPACT_SOUND_VOLUME,
                            looped: false
                        },
                    );

                    let da = u.rotation - p.rotation;
                    p.alive = false;
                    u.rotation += (da.abs() / da) * f32::to_radians(20.);
                }
            }
        }
    }

    /// отрисовка
    pub fn draw(&self) {
        self.target_unit.draw_shadow();
        self.main_unit.draw();
        for i in 0..self.enemy_units.len() {
            self.enemy_units[i].draw_shadow();
            self.enemy_units[i].draw();
        }
        for i in 0..self.projectiles.len() {
            self.projectiles[i].draw();
        }
        self.target_unit.draw();
    }
}
