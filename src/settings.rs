mod themes;

use crate::settings::themes::{CATPPUCCIN_MACCHIATO, DEFAULT_THEME, THEMES, Theme};

use log::warn;
use serde::{Deserialize, Serialize};

pub struct Settings {
    pub symbols: bool,
    pub theme: Theme,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SettingsFile {
    pub symbols: Option<bool>,
    pub theme: Option<String>,
}

impl Settings {
    pub fn load_from_config() -> Self {
        let root_settings = match std::fs::read_to_string("/etc/curlpp/config.toml") {
            Ok(root_settings) => toml::from_str::<SettingsFile>(&root_settings).unwrap_or_default(),
            Err(_) => {
                warn!("Root config file not present");
                Default::default()
            }
        };

        let user_settings = match dirs::home_dir().and_then(|h| h.to_str().map(str::to_string)) {
            Some(home_dir) => {
                match std::fs::read_to_string(format!("{}/curlpp/config.toml", home_dir)) {
                    Ok(user_settings) => {
                        toml::from_str::<SettingsFile>(&user_settings).unwrap_or_default()
                    }
                    Err(_) => {
                        warn!("User config file not present");
                        Default::default()
                    }
                }
            }
            None => {
                warn!("Home directory env var not set");
                Default::default()
            }
        };

        let default_symbols = false;

        let theme = user_settings
            .theme
            .and_then(|t| THEMES.get(t.as_str()))
            .unwrap_or(
                root_settings
                    .theme
                    .and_then(|t| THEMES.get(t.as_str()))
                    .unwrap_or(THEMES.get(DEFAULT_THEME).unwrap_or(&CATPPUCCIN_MACCHIATO)),
            );

        Settings {
            symbols: user_settings
                .symbols
                .unwrap_or(root_settings.symbols.unwrap_or(default_symbols)),
            theme: *theme,
        }
    }
}
