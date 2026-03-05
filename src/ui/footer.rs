//! Footer bar with keybind hints and status

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

use crate::app::state::{Notification, Page};
use crate::ui::theme::ThemeColors;

/// Footer bar widget
pub struct Footer<'a> {
    page: Page,
    sample_rate: Option<u32>,
    notification: Option<&'a Notification>,
    colors: ThemeColors,
}

impl<'a> Footer<'a> {
    pub fn new(page: Page, colors: ThemeColors) -> Self {
        Self {
            page,
            sample_rate: None,
            notification: None,
            colors,
        }
    }

    pub fn sample_rate(mut self, rate: Option<u32>) -> Self {
        self.sample_rate = rate;
        self
    }

    pub fn notification(mut self, notification: Option<&'a Notification>) -> Self {
        self.notification = notification;
        self
    }

    fn keybinds(&self) -> Vec<(&'static str, &'static str)> {
        let mut binds = vec![
            ("q", "Quit"),
            ("p/Space", "Pause"),
            ("h", "Prev"),
            ("l", "Next"),
            ("t", "Theme"),
            ("-/+", "Volume")
        ];

        match self.page {
            Page::Artists => {
                binds.extend([
                    ("/", "Filter"),
                    ("←/→", "Focus"),
                    ("e", "Add"),
                    ("n", "Add next"),
                    ("Enter", "Play"),
                ]);
            }
            Page::Queue => {
                binds.extend([
                    ("d", "Remove"),
                    ("J/K", "Move"),
                    ("r", "Shuffle"),
                    ("c", "Clear history"),
                    ("Enter", "Play"),
                ]);
            }
            Page::Playlists => {
                binds.extend([
                    ("←/→", "Focus"),
                    ("e", "Add"),
                    ("n", "Add next"),
                    ("r", "Shuffle play"),
                    ("Enter", "Play"),
                ]);
            }
            Page::Server => {
                binds.extend([
                    ("Tab", "Next field"),
                    ("Enter", "Test/Save"),
                    ("Ctrl+R", "Refresh"),
                ]);
            }
            Page::Settings => {
                binds.extend([("←/→/Enter", "Change theme")]);
            }
        }

        binds
    }
}

impl Widget for Footer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 {
            return;
        }

        // Only reserve space on the right side when there's a sample rate to show
        let rate_str = self.sample_rate.map(|r| format!("{}kHz", r / 1000));
        let right_width = rate_str.as_ref().map(|s| s.len() as u16 + 1).unwrap_or(0);

        let chunks = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(right_width),
        ])
        .split(area);

        // Left side: keybinds or notification
        if let Some(notif) = self.notification {
            let style = if notif.is_error {
                Style::default().fg(self.colors.error)
            } else {
                Style::default().fg(self.colors.success)
            };
            buf.set_string(chunks[0].x, chunks[0].y, &notif.message, style);
        } else {
            // Keybind hints
            let binds = self.keybinds();
            let mut spans = Vec::new();

            for (i, (key, desc)) in binds.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::styled(
                        " │ ",
                        Style::default().fg(self.colors.secondary),
                    ));
                }
                spans.push(Span::styled(*key, Style::default().fg(self.colors.accent)));
                spans.push(Span::raw(":"));
                spans.push(Span::styled(*desc, Style::default().fg(self.colors.muted)));
            }

            let line = Line::from(spans);
            buf.set_line(chunks[0].x, chunks[0].y, &line, chunks[0].width);
        }

        // Right side: sample rate
        if let Some(rate_str) = rate_str {
            let x = chunks[1].x + chunks[1].width.saturating_sub(rate_str.len() as u16);
            buf.set_string(x, chunks[1].y, &rate_str, Style::default().fg(self.colors.success));
        }
    }
}
