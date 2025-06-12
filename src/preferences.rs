use directories::ProjectDirs;
use once_cell::sync::Lazy;
use std::{error::Error, fs, path::PathBuf, sync::Arc};

use iced::{Theme, theme::Custom};
use serde::{Deserialize, Serialize};

pub static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("org", "cambio", "torrefaction").unwrap());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(with = "ThemeDef")]
    pub theme: Theme,
}

#[derive(Deserialize, Serialize)]
#[serde(remote = "Theme")]
enum ThemeDef {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
    #[serde(skip)]
    Custom(Arc<Custom>),
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            theme: Theme::TokyoNight,
        }
    }
}

impl Preferences {
    fn config_file() -> PathBuf {
        let mut path = PROJECT_DIRS.preference_dir().join("_").to_path_buf();
        path.set_file_name("config.toml");
        path
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let preferences = Preferences::default();
        if let Ok(string) = fs::read_to_string(Self::config_file()) {
            Ok(toml::from_str(&string)?)
        } else {
            preferences.save()?;
            Ok(preferences)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::config_file();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string(&self)?)?;
        Ok(())
    }
}
