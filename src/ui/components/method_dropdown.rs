use crate::{state::Method, Application};

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

impl Application {
    pub fn render_method_dropdown(&self, frame: &mut Frame, area: Rect) {
        let (get_fg, get_bg) = if self.method_state.current_method == Method::Get {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.get_color,
            )
        } else {
            (self.settings.theme.get_color, self.settings.theme.inactive)
        };

        let (post_fg, post_bg) = if self.method_state.current_method == Method::Post {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.post_color,
            )
        } else {
            (self.settings.theme.post_color, self.settings.theme.inactive)
        };

        let (put_fg, put_bg) = if self.method_state.current_method == Method::Put {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.put_color,
            )
        } else {
            (self.settings.theme.put_color, self.settings.theme.inactive)
        };

        let (patch_fg, patch_bg) = if self.method_state.current_method == Method::Patch {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.patch_color,
            )
        } else {
            (
                self.settings.theme.patch_color,
                self.settings.theme.inactive,
            )
        };

        let (options_fg, options_bg) = if self.method_state.current_method == Method::Options {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.options_color,
            )
        } else {
            (
                self.settings.theme.options_color,
                self.settings.theme.inactive,
            )
        };

        let (connect_fg, connect_bg) = if self.method_state.current_method == Method::Connect {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.connect_color,
            )
        } else {
            (
                self.settings.theme.connect_color,
                self.settings.theme.inactive,
            )
        };

        let (trace_fg, trace_bg) = if self.method_state.current_method == Method::Trace {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.trace_color,
            )
        } else {
            (
                self.settings.theme.trace_color,
                self.settings.theme.inactive,
            )
        };

        let (delete_fg, delete_bg) = if self.method_state.current_method == Method::Delete {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.delete_color,
            )
        } else {
            (
                self.settings.theme.delete_color,
                self.settings.theme.inactive,
            )
        };

        let (head_fg, head_bg) = if self.method_state.current_method == Method::Head {
            (
                self.settings.theme.text_inverse,
                self.settings.theme.head_color,
            )
        } else {
            (self.settings.theme.head_color, self.settings.theme.inactive)
        };

        let dropdown_contents = Paragraph::new(vec![
            Line::from(Span::styled(
                "▄▄▄▄▄▄▄▄▄▄▄▄",
                Style::default().fg(self.settings.theme.inactive_element),
            )),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(get_bg)),
                Span::styled("Get", Style::default().fg(get_fg).bg(get_bg)),
                Span::styled("██████", Style::default().fg(get_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(post_bg)),
                Span::styled("Post", Style::default().fg(post_fg).bg(post_bg)),
                Span::styled("█████", Style::default().fg(post_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(put_bg)),
                Span::styled("Put", Style::default().fg(put_fg).bg(put_bg)),
                Span::styled("██████", Style::default().fg(put_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(patch_bg)),
                Span::styled("Patch", Style::default().fg(patch_fg).bg(patch_bg)),
                Span::styled("████", Style::default().fg(patch_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(options_bg)),
                Span::styled("Options", Style::default().fg(options_fg).bg(options_bg)),
                Span::styled("██", Style::default().fg(options_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(connect_bg)),
                Span::styled("Connect", Style::default().fg(connect_fg).bg(connect_bg)),
                Span::styled("██", Style::default().fg(connect_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(trace_bg)),
                Span::styled("Trace", Style::default().fg(trace_fg).bg(trace_bg)),
                Span::styled("████", Style::default().fg(trace_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(delete_bg)),
                Span::styled("Delete", Style::default().fg(delete_fg).bg(delete_bg)),
                Span::styled("███", Style::default().fg(delete_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from_iter([
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
                Span::styled("█", Style::default().fg(head_bg)),
                Span::styled("Head", Style::default().fg(head_fg).bg(head_bg)),
                Span::styled("█████", Style::default().fg(head_bg)),
                Span::styled(
                    "█",
                    Style::default().fg(self.settings.theme.inactive_element),
                ),
            ]),
            Line::from(Span::styled(
                "▀▀▀▀▀▀▀▀▀▀▀▀",
                Style::default().fg(self.settings.theme.inactive_element),
            )),
        ]);
        frame.render_widget(dropdown_contents, area);
    }
}
