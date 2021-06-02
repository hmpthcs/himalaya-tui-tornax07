use super::mail_credits::MailCredits;

use crate::config::model::Config;
use crate::config::tui::TuiConfig;
use crate::tui::model::BackendActions;
use crate::tui::modes::{
    backend_interface::BackendInterface, keybinding_manager::KeybindingManager,
};

use tui_rs::backend::Backend;
use tui_rs::terminal::Frame;
use tui_rs::layout::{Layout, Direction, Constraint};

use crossterm::event::Event;

// use crate::msg::tpl::model::Tpl;

// ==========
// Enums
// ==========
#[derive(Clone)]
pub enum WritingMailAction {
    SetBcc,
    SetCc,
    SetInReplyTo,
    SetSubject,
    SetTo,
    Quit,
}

// ============
// Structs
// ============
pub struct WritingMail {
    credits: MailCredits,
    // template: Tpl,
    keybinding_manager: KeybindingManager<WritingMailAction>,
}

impl WritingMail {
    pub fn new(config: &Config) -> Self {
        // ----------------
        // Keybindings
        // ----------------
        let default_keybindings = vec![
            ("set_bcc", WritingMailAction::SetBcc, "b"),
            ("set_cc", WritingMailAction::SetBcc, "c"),
            ("set_in_reply_to", WritingMailAction::SetBcc, "r"),
            ("set_subject", WritingMailAction::SetBcc, "s"),
            ("set_to", WritingMailAction::SetBcc, "t"),
            ("quit", WritingMailAction::Quit, "q"),
        ];

        let keybindings = TuiConfig::parse_keybindings(
            &default_keybindings,
            config.tui.keybindings.get("writing_mail"),
        );

        let credits = MailCredits::new(
            String::from("tornax07@gmail.com"),
            &config.tui.mail_credits
        );

        Self {
            // template: Tpl::new(),
            credits,
            keybinding_manager: KeybindingManager::new(keybindings),
        }
    }
}

impl BackendInterface for WritingMail {
    fn handle_event(&mut self, event: Event) -> Option<BackendActions> {
        if let Some(action) = self.keybinding_manager.eval_event(event) {
            match action {
                WritingMailAction::Quit => Some(BackendActions::Quit),
                _ => None,
            }
        } else {
            None
        }
    }

    fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    // At the top the mail credits
                    Constraint::Percentage(25),
                    // At the bottom the listing
                    Constraint::Percentage(75),
                ]
                .as_ref(),
            )
            .split(frame.size());

        frame.render_widget(self.credits.widget(), layout[0]);

    }
}
