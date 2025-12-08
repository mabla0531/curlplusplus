use crate::{state::Method, ui::palette, Application};

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
            (palette::CRUST, palette::GET_COLOR)
        } else {
            (palette::GET_COLOR, palette::SURFACE1)
        };

        let (post_fg, post_bg) = if self.method_state.current_method == Method::Post {
            (palette::CRUST, palette::POST_COLOR)
        } else {
            (palette::POST_COLOR, palette::SURFACE1)
        };

        let (put_fg, put_bg) = if self.method_state.current_method == Method::Put {
            (palette::CRUST, palette::PUT_COLOR)
        } else {
            (palette::PUT_COLOR, palette::SURFACE1)
        };

        let (patch_fg, patch_bg) = if self.method_state.current_method == Method::Patch {
            (palette::CRUST, palette::PATCH_COLOR)
        } else {
            (palette::PATCH_COLOR, palette::SURFACE1)
        };

        let (options_fg, options_bg) = if self.method_state.current_method == Method::Options {
            (palette::CRUST, palette::OPTIONS_COLOR)
        } else {
            (palette::OPTIONS_COLOR, palette::SURFACE1)
        };

        let (connect_fg, connect_bg) = if self.method_state.current_method == Method::Connect {
            (palette::CRUST, palette::CONNECT_COLOR)
        } else {
            (palette::CONNECT_COLOR, palette::SURFACE1)
        };

        let (trace_fg, trace_bg) = if self.method_state.current_method == Method::Trace {
            (palette::CRUST, palette::TRACE_COLOR)
        } else {
            (palette::TRACE_COLOR, palette::SURFACE1)
        };

        let (delete_fg, delete_bg) = if self.method_state.current_method == Method::Delete {
            (palette::CRUST, palette::DELETE_COLOR)
        } else {
            (palette::DELETE_COLOR, palette::SURFACE1)
        };

        let (head_fg, head_bg) = if self.method_state.current_method == Method::Head {
            (palette::CRUST, palette::HEAD_COLOR)
        } else {
            (palette::HEAD_COLOR, palette::SURFACE1)
        };

        let dropdown_contents = Paragraph::new(vec![
            Line::from(Span::styled(
                "▄▄▄▄▄▄▄▄▄▄▄▄",
                Style::default().fg(palette::SURFACE0),
            )),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(get_bg)),
                Span::styled("Get", Style::default().fg(get_fg).bg(get_bg)),
                Span::styled("██████", Style::default().fg(get_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(post_bg)),
                Span::styled("Post", Style::default().fg(post_fg).bg(post_bg)),
                Span::styled("█████", Style::default().fg(post_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(put_bg)),
                Span::styled("Put", Style::default().fg(put_fg).bg(put_bg)),
                Span::styled("██████", Style::default().fg(put_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(patch_bg)),
                Span::styled("Patch", Style::default().fg(patch_fg).bg(patch_bg)),
                Span::styled("████", Style::default().fg(patch_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(options_bg)),
                Span::styled("Options", Style::default().fg(options_fg).bg(options_bg)),
                Span::styled("██", Style::default().fg(options_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(connect_bg)),
                Span::styled("Connect", Style::default().fg(connect_fg).bg(connect_bg)),
                Span::styled("██", Style::default().fg(connect_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(trace_bg)),
                Span::styled("Trace", Style::default().fg(trace_fg).bg(trace_bg)),
                Span::styled("████", Style::default().fg(trace_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(delete_bg)),
                Span::styled("Delete", Style::default().fg(delete_fg).bg(delete_bg)),
                Span::styled("███", Style::default().fg(delete_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(head_bg)),
                Span::styled("Head", Style::default().fg(head_fg).bg(head_bg)),
                Span::styled("█████", Style::default().fg(head_bg)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from(Span::styled(
                "▀▀▀▀▀▀▀▀▀▀▀▀",
                Style::default().fg(palette::SURFACE0),
            )),
        ]);
        frame.render_widget(dropdown_contents, area);
    }
}
