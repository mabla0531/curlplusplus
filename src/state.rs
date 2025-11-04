pub struct AppState {
    pub focused_panel: FocusedPanel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FocusedPanel {
    Method { show_dropdown: bool },
    Url,
    Request,
    Response,
}
