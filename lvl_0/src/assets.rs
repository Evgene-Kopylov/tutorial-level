//! ассеты с предворительной загрузкой

use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::{info};
use macroquad::texture::{load_texture, Texture2D};
use macroquad::Error;

/// Набор ассетов. Текстуры и звуки.
pub(crate) struct Assets {
    pub(crate) main_unit_texture: Texture2D,
    pub(crate) main_unit_shoot_sound: Sound,
    pub(crate) target_impact_sound: Sound,
    pub(crate) target_unit_texture: Texture2D,
    pub(crate) target_unit_shadow_texture: Texture2D,
    pub(crate) projectile_texture: Texture2D,
    pub(crate) enemy_unit_gray: Texture2D,
}

impl Assets {
    /// Читать файлы ассетов
    pub async fn new() -> Result<Self, Error> {
        info!("WASM LOG: Начало загрузки текстур");
        let main_unit_texture: Texture2D = load_texture("../assets/pointer/pointer_3.png").await?;
        let main_unit_shoot_sound: Sound = load_sound("../assets/sound/4 XSA_Weapon.wav").await?;
        let target_impact_sound: Sound =
            load_sound("../assets/sound/hit-with-something.wav").await?;
        let target_unit_texture = load_texture("../assets/pointer/target_unit_3_2.png").await?;
        let target_unit_shadow_texture =
            load_texture("../assets/pointer/target_unit_3_shadow.png").await?;
        let enemy_unit_gray = load_texture("../assets/pointer/enemy_unit_gray.png").await?;
        let projectile_texture = load_texture("../assets/pointer/projectile_glow_7.png").await?;
        info!("WASM LOG: Текстуры загружены");

        Ok(Self {
            main_unit_texture,
            main_unit_shoot_sound,
            target_impact_sound,
            target_unit_texture,
            target_unit_shadow_texture,
            enemy_unit_gray,
            projectile_texture,
        })
    }
}
