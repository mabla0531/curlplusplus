pub mod animations;
pub mod components;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Block,
};

use crate::{Application, client::WrappedResponse, state::Panel};

struct LayoutSet {
    pub app_name: Rect,
    pub method_selector: Rect,
    pub method_dropdown: Rect,
    pub url_input: Rect,
    pub main_pane: Rect,
    pub help_bar: Rect,
}

fn layout(frame: &mut Frame) -> LayoutSet {
    let page_vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let top_bar = Layout::horizontal([
        Constraint::Length(8),
        Constraint::Length(13),
        Constraint::Fill(1),
    ])
    .split(page_vertical[0]);

    let method_dropdown = Rect {
        x: top_bar[1].x + 1,
        y: top_bar[1].y + 2,
        width: 12,
        height: 11,
    };

    LayoutSet {
        app_name: top_bar[0],
        method_selector: top_bar[1],
        method_dropdown,
        url_input: top_bar[2],
        main_pane: page_vertical[1],
        help_bar: page_vertical[2],
    }
}

impl Application {
    pub fn render(&mut self, frame: &mut Frame) {
        let layout = layout(frame);

        frame.render_widget(Block::new().bg(self.settings.theme.base), frame.area());

        self.render_title(frame, layout.app_name);
        self.render_method(frame, layout.method_selector);
        self.render_url_input(frame, layout.url_input);
        self.render_main_pane(frame, layout.main_pane);
        self.render_help_bar(frame, layout.help_bar);

        if self.editing && self.focused_panel == Panel::Method {
            // has to be after all others since it's a "z-index: 1;" element
            self.render_method_dropdown(frame, layout.method_dropdown);
        }
    }

    pub fn response_status_pane_height(&self) -> u16 {
        self.terminal_height.saturating_sub(9)
    }
}

pub fn response_status_height(terminal_width: u16, response: &WrappedResponse) -> usize {
    let headers_length = response.meta.headers.len();

    let body_length: usize = response
        .body
        .lines()
        .map(|line| {
            ((line.chars().count().max(1) as f64) / (terminal_width.saturating_sub(4) as f64))
                .ceil() as usize
        })
        .sum::<usize>()
        .max(1);

    //url/status + 1 gap + headers label + headers (each gapped) + body label + body_length
    1 + 1 + 1 + (headers_length * 2) + 1 + body_length
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use reqwest::Url;

    use crate::client::{BodyStatus, WrappedResponseMeta};

    use super::*;

    #[test]
    fn test_billion_of_header() {
        let meta = WrappedResponseMeta {
            url: Url::from_str("http://localhost").unwrap(),
            content_type: Default::default(),
            headers: vec![
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "so no head? : wrong, this is HTTP, we have plenty of head here".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "my_extremely_long_header : aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
                "".into(),
            ],
            status: Default::default(),
        };

        assert_eq!(
            response_status_height(
                22,
                &WrappedResponse {
                    meta,
                    body: "".into(),
                    body_status: BodyStatus::Finished
                }
            ),
            89
        );
    }

    #[test]
    fn test_comical_body() {
        let empty_meta = WrappedResponseMeta {
            url: Url::from_str("http://localhost").unwrap(),
            content_type: Default::default(),
            headers: Default::default(),
            status: Default::default(),
        };

        let body = [
            "asdfasdfasdfasdfasdf",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "asdfasdfasdf",
            "",
            "",
            "",
            "",
            "asdfasdfasdf",
            "",
            "",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "asdfasdfasdf",
            "123456789012345678901234567890123456",
        ].join("\n");

        let body_length: usize = body
            .lines()
            .map(|line| ((line.len().max(1) as f64) / 18_f64).ceil() as usize)
            .sum::<usize>()
            .max(1);

        assert_eq!(body_length, 81);

        assert_eq!(
            response_status_height(
                22,
                &WrappedResponse {
                    meta: empty_meta.clone(),
                    body,
                    body_status: BodyStatus::Finished,
                }
            ),
            85
        );
    }

    #[test]
    fn test_status_height() {
        let empty_meta = WrappedResponseMeta {
            url: Url::from_str("http://localhost").unwrap(),
            content_type: Default::default(),
            headers: Default::default(),
            status: Default::default(),
        };

        assert_eq!(
            response_status_height(
                22,
                &WrappedResponse {
                    meta: empty_meta.clone(),
                    body: "".to_string(),
                    body_status: BodyStatus::Finished,
                }
            ),
            5
        );

        assert_eq!(
            response_status_height(
                22,
                &WrappedResponse {
                    meta: empty_meta.clone(),
                    body: "aaaaaaaaaaaaaaaaaaaa".to_string(),
                    body_status: BodyStatus::Finished,
                }
            ),
            6
        );

        assert_eq!(
            response_status_height(
                22,
                &WrappedResponse {
                    meta: empty_meta.clone(),
                    body: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                    body_status: BodyStatus::Finished,
                }
            ),
            7
        );

        assert_eq!(
            response_status_height(
                22,
                &WrappedResponse {
                    meta: WrappedResponseMeta {
                        headers: vec!["header1 : myvalue".into(), "my_extremely_long_header : aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()],
                        ..empty_meta.clone()
                    },
                    body: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                    body_status: BodyStatus::Finished,
                }
            ),
            11
        );
    }
}
