use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

use crate::{
    Application, ResponseType, client::WrappedResponse, errors::SendRequestError,
    ui::response_status_height,
};

impl Application {
    pub fn render_response_status_pane(&mut self, frame: &mut Frame, area: Rect) {
        let last_response = self.last_response.clone();
        let last_response = last_response.lock().unwrap();
        let last_response = last_response.as_ref();

        match last_response {
            ResponseType::None => self.render_response_status_none(frame, area),
            ResponseType::Pending => self.render_response_status_pending(frame, area),
            ResponseType::FinishedSuccess(response) => {
                self.render_response_status_success(response, frame, area)
            }
            ResponseType::FinishedError(error) => {
                self.render_response_status_error(error, frame, area)
            }
        }
    }

    pub fn render_response_status_none(&self, frame: &mut Frame, area: Rect) {
        let vertical_center = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area)[1];

        let paragraph =
            Paragraph::new("Send a request to see the result here").alignment(Alignment::Center);

        frame.render_widget(paragraph, vertical_center);
    }

    pub fn render_response_status_pending(&mut self, frame: &mut Frame, area: Rect) {
        let vertical_center = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(self.animation_state.loading_spinner.frame_height as u16),
            Constraint::Fill(1),
        ])
        .split(area)[1];

        self.animation_state.loading_spinner.tick();
        let loading_spinner_lines = self.animation_state.loading_spinner.render();

        let paragraph = Paragraph::new(Text::from(loading_spinner_lines)).centered();

        frame.render_widget(paragraph, vertical_center);
    }

    pub fn render_response_status_success(
        &self,
        response: &WrappedResponse,
        frame: &mut Frame,
        area: Rect,
    ) {
        let body_icon = response.body_status.icon();

        let status_color = match response.meta.status.as_u16() {
            100..200 => self.settings.theme.active,
            200..300 => self.settings.theme.green,
            300..400 => self.settings.theme.accent,
            400..500 => self.settings.theme.red,
            500..600 => self.settings.theme.red,
            _ => self.settings.theme.active,
        };

        let url = self.badge(
            format!("URL: {}", response.meta.url),
            None,
            self.settings.theme.base,
        );

        let code = self.badge(
            format!("Status Code: {}", response.meta.status),
            Some(self.settings.theme.text_inverse),
            status_color,
        );

        let url_code_line = vec![
            Line::from_iter([url, vec![Span::from(" ")], code].concat()),
            Line::from(""),
        ];

        let headers_lines = [
            vec![Line::from("Headers:")],
            response
                .meta
                .headers
                .iter()
                .cloned()
                .map(|header| {
                    let header = if header.len() > self.terminal_width.saturating_sub(4) as usize {
                        format!(
                            "{}...",
                            &header[0..(self.terminal_width.saturating_sub(4) as usize)]
                                .to_string()
                        )
                    } else {
                        header
                    };
                    Line::from(Span::from(header))
                })
                .collect::<Vec<_>>(),
            vec![Line::from("")],
        ]
        .concat();

        let body_lines = [
            vec![Line::from(format!("Body{}:", body_icon))],
            response
                .body
                .lines()
                .flat_map(|line| {
                    if line.is_empty() {
                        vec![Line::from("")]
                    } else {
                        line.chars()
                            .collect::<Vec<_>>()
                            .chunks(self.terminal_width.saturating_sub(4) as usize)
                            .map(|chunk| Line::from(chunk.iter().collect::<String>()))
                            .collect::<Vec<_>>()
                    }
                })
                .collect(),
        ]
        .concat();

        let response_lines = [url_code_line, headers_lines, body_lines].concat();
        let safe_pane_height = (self.main_state.response_status_scroll
            + self.response_status_pane_height() as usize)
            .min(response_status_height(self.terminal_width, response))
            .min(response_lines.len()); // hard bail

        let viewable_response_lines =
            response_lines.get(self.main_state.response_status_scroll..safe_pane_height);

        let viewable_response_lines = viewable_response_lines
            .map(Into::into)
            .unwrap_or(response_lines);

        let response_text = Text::from_iter(viewable_response_lines);
        let paragraph = Paragraph::new(response_text)
            .block(Block::default().padding(Padding::uniform(1)))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    pub fn render_response_status_error(
        &self,
        error: &SendRequestError,
        frame: &mut Frame,
        area: Rect,
    ) {
        let vertical_center = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area)[1];

        let paragraph = Paragraph::new(Span::styled(
            format!("Failed to send request: {}", error),
            Style::default().fg(self.settings.theme.red),
        ))
        .centered();

        frame.render_widget(paragraph, vertical_center);
    }
}
