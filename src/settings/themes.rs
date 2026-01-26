use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs::{DirEntry, ReadDir};
use std::{collections::HashMap, sync::LazyLock};

/// Colors are formatted as "#ffffff"
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub accent: Color,
    pub green: Color,
    pub red: Color,
    pub active: Color,
    pub inactive: Color,
    pub text: Color,
    pub text_inverse: Color,
    pub active_text: Color,
    pub inactive_text: Color,
    pub active_element: Color,
    pub inactive_element: Color,
    pub base: Color,
    pub get_color: Color,
    pub post_color: Color,
    pub put_color: Color,
    pub patch_color: Color,
    pub options_color: Color,
    pub connect_color: Color,
    pub trace_color: Color,
    pub delete_color: Color,
    pub head_color: Color,
}

/// if any HTTP request type colors are not specified, the content types from catppuccin macchiato
/// will be used
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ThemeFile {
    pub accent: Color,
    pub green: Color,
    pub red: Color,
    pub active: Color,
    pub inactive: Color,
    pub text: Color,
    pub text_inverse: Color,
    pub active_text: Color,
    pub inactive_text: Color,
    pub active_element: Color,
    pub inactive_element: Color,
    pub base: Color,
    pub get_color: Option<Color>,
    pub post_color: Option<Color>,
    pub put_color: Option<Color>,
    pub patch_color: Option<Color>,
    pub options_color: Option<Color>,
    pub connect_color: Option<Color>,
    pub trace_color: Option<Color>,
    pub delete_color: Option<Color>,
    pub head_color: Option<Color>,
}

impl From<ThemeFile> for Theme {
    fn from(val: ThemeFile) -> Theme {
        Theme {
            accent: val.accent,
            green: val.green,
            red: val.red,
            active: val.active,
            inactive: val.inactive,
            text: val.text,
            text_inverse: val.text_inverse,
            active_text: val.active_text,
            inactive_text: val.inactive_text,
            active_element: val.active_element,
            inactive_element: val.inactive_element,
            base: val.base,
            get_color: val.get_color.unwrap_or(CATPPUCCIN_MACCHIATO.get_color),
            post_color: val.post_color.unwrap_or(CATPPUCCIN_MACCHIATO.post_color),
            put_color: val.put_color.unwrap_or(CATPPUCCIN_MACCHIATO.put_color),
            patch_color: val.patch_color.unwrap_or(CATPPUCCIN_MACCHIATO.patch_color),
            options_color: val
                .options_color
                .unwrap_or(CATPPUCCIN_MACCHIATO.options_color),
            connect_color: val
                .connect_color
                .unwrap_or(CATPPUCCIN_MACCHIATO.connect_color),
            trace_color: val.trace_color.unwrap_or(CATPPUCCIN_MACCHIATO.trace_color),
            delete_color: val
                .delete_color
                .unwrap_or(CATPPUCCIN_MACCHIATO.delete_color),
            head_color: val.head_color.unwrap_or(CATPPUCCIN_MACCHIATO.head_color),
        }
    }
}

pub static THEMES: LazyLock<HashMap<String, Theme>> =
    LazyLock::new(|| HashMap::from_iter(load_themes()));

fn load_theme_from_file(file: DirEntry) -> Option<(String, Theme)> {
    let contents = std::fs::read_to_string(file.path()).ok()?;

    let theme_name = file.path();
    let theme_name = theme_name.file_stem()?.to_str();

    let theme = toml::from_str::<ThemeFile>(&contents).ok();

    Some(theme_name?.to_string()).zip(theme.map(Into::into))
}

fn load_themes_from_files() -> Vec<(String, Theme)> {
    let root_themes = if let Ok(theme_files) = std::fs::read_dir("/etc/curlpp/themes/") {
        theme_files
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(load_theme_from_file)
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let user_themes = match dirs::home_dir().and_then(|h| h.to_str().map(str::to_string)) {
        Some(home_dir) => {
            if let Ok(theme_files) = std::fs::read_dir(format!("{}/curlpp/themes", home_dir)) {
                theme_files
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter_map(load_theme_from_file)
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    [root_themes, user_themes].concat()
}

fn load_themes() -> Vec<(String, Theme)> {
    let custom_themes = load_themes_from_files();
    let builtin_themes = vec![
        (DEFAULT_THEME.to_string(), CATPPUCCIN_MACCHIATO),
        ("dracula".to_string(), DRACULA),
    ];

    [builtin_themes, custom_themes].concat()
}

pub const DEFAULT_THEME: &str = "dracula";

pub const CATPPUCCIN_MACCHIATO: Theme = Theme {
    accent: Color::Rgb(245, 169, 127),
    green: Color::Rgb(166, 218, 149),
    red: Color::Rgb(237, 135, 150),
    active: Color::Rgb(145, 215, 227),
    inactive: Color::Rgb(73, 77, 100),
    text: Color::Rgb(202, 211, 245),
    text_inverse: Color::Rgb(30, 32, 48),
    active_text: Color::Rgb(184, 192, 224),
    inactive_text: Color::Rgb(165, 173, 203),
    active_element: Color::Rgb(91, 96, 120),
    inactive_element: Color::Rgb(54, 58, 79),
    base: Color::Rgb(30, 32, 48),
    get_color: Color::Rgb(145, 215, 227),
    post_color: Color::Rgb(244, 219, 214),
    put_color: Color::Rgb(139, 213, 202),
    patch_color: Color::Rgb(198, 160, 246),
    options_color: Color::Rgb(245, 169, 127),
    connect_color: Color::Rgb(238, 212, 159),
    trace_color: Color::Rgb(238, 153, 160),
    delete_color: Color::Rgb(237, 135, 150),
    head_color: Color::Rgb(166, 218, 149),
};

pub const DRACULA: Theme = Theme {
    accent: Color::Rgb(98, 114, 164),
    green: Color::Rgb(80, 250, 123),
    red: Color::Rgb(255, 85, 85),
    active: Color::Rgb(98, 114, 164),
    inactive: Color::Rgb(68, 71, 90),
    text: Color::Rgb(248, 248, 242),
    text_inverse: Color::Rgb(40, 42, 54),
    active_text: Color::Rgb(248, 248, 242),
    inactive_text: Color::Rgb(248, 248, 242),
    active_element: Color::Rgb(98, 114, 164),
    inactive_element: Color::Rgb(68, 71, 90),
    base: Color::Rgb(40, 42, 54),
    get_color: Color::Rgb(139, 233, 253),
    post_color: Color::Rgb(189, 147, 249),
    put_color: Color::Rgb(255, 184, 108),
    patch_color: Color::Rgb(241, 250, 140),
    options_color: Color::Rgb(255, 121, 198),
    connect_color: Color::Rgb(80, 250, 123),
    trace_color: Color::Rgb(98, 114, 164),
    delete_color: Color::Rgb(255, 85, 85),
    head_color: Color::Rgb(248, 248, 242),
};
