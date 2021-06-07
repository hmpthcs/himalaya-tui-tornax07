use crate::config::tui::block_data::BlockDataConfig;
use crate::tui::modes::normal::main::NormalAction;

use serde::Deserialize;

use std::collections::HashMap;

// ============
// Structs
// ============
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct NormalConfig {
    pub sidebar: BlockDataConfig,
    pub mail_list: BlockDataConfig,
    pub keybindings: HashMap<String, String>,

    #[serde(skip, default = "NormalConfig::default_keybindings")]
    pub default_keybindings: Vec<(&'static str, NormalAction, &'static str)>,
}

impl Default for NormalConfig {
    fn default() -> Self {
        Self {
            sidebar: BlockDataConfig::default(),
            mail_list: BlockDataConfig::default(),
            keybindings: HashMap::new(),
            default_keybindings: NormalConfig::default_keybindings(),
        }
    }
}

impl NormalConfig {

    fn default_keybindings() -> Vec<(&'static str, NormalAction, &'static str)> {
        vec![
            ("quit", NormalAction::Quit, "q"),
            ("cursor_down", NormalAction::CursorOffset(1), "j"),
            ("cursor_up", NormalAction::CursorOffset(-1), "k"),
            ("new_mail", NormalAction::WritingMail, "m"),
            ("toggle_sidebar", NormalAction::ToggleSidebar, "<C-b>"),
            ("go_top", NormalAction::CursorAbsolut(Some(0)), "gg"),
            ("go_bottom", NormalAction::CursorAbsolut(None), "<S-g>"),
            ("viewer", NormalAction::ViewingMail, "l"),
        ]
    }
}