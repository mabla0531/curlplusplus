use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Span, Text},
    widgets::Paragraph,
};

use crate::{Application, ResponseType, client::WrappedResponse, errors::SendRequestError};

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

        let paragraph = Paragraph::new(format!(
            "URL: {}\n\nStatus Code: {}\n\nHeaders: {}\n\nBody{}:\n\n{}",
            response.meta.url,
            response.meta.status,
            response
                .meta
                .headers
                .iter()
                .map(|(name, value)| format!(
                    "\t{} : {}",
                    name,
                    value.to_str().unwrap_or("invalid")
                ))
                .collect::<Vec<String>>()
                .join("\n"),
            body_icon,
            response.body
        ));

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
