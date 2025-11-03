pub struct AppState {
    pub focused_panel: FocusedPanel,
}

pub enum FocusedPanel {
    Method,
    Url,
    Request,
    Response,
}
