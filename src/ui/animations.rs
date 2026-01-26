use ratatui::text::Line;

// ▔ ▀ ▄ ▁ ▂ ▃ ▅ ▆ ▇ █
// ◤ ◥ ◣ ◢ ◥ ◤
// ▌ ▐
// ▘ ▝ ▖ ▗

const LOADING_SPINNER_ANIMATION: Animation = Animation {
    frame_set: FrameSet {
        frames: &[
            " ◢███   █◣\n◢█◤     ◥█◣\n██       ██\n◥█◣     ◢█◤\n ◥███████◤",
            " ◢█████   \n◢█◤        \n██       ██\n◥█◣     ◢█◤\n ◥███████◤",
            " ◢███████◣\n◢█◤     ◥█◣\n██         \n◥█◣     ◢█◤\n ◥███████◤",
            " ◢███████◣\n◢█◤     ◥█◣\n██       ██\n◥█◣        \n ◥█████   ",
            " ◢███████◣\n◢█◤     ◥█◣\n██       ██\n◥█◣     ◢█◤\n ◥███   █◤",
            " ◢███████◣\n◢█◤     ◥█◣\n██       ██\n◥█◣     ◢█◤\n ◥█   ███◤",
            " ◢███████◣\n◢█◤     ◥█◣\n██       ██\n        ◢█◤\n    █████◤",
            " ◢███████◣\n◢█◤     ◥█◣\n         ██\n◥█◣     ◢█◤\n ◥███████◤",
            "    █████◣\n        ◥█◣\n██       ██\n◥█◣     ◢█◤\n ◥███████◤",
            " ◢█   ███◣\n◢█◤     ◥█◣\n██       ██\n◥█◣     ◢█◤\n ◥███████◤",
        ],
        length: 10,
    },
    frame: 0,
    frame_height: 5,
};

#[derive(Clone)]
pub struct FrameSet {
    pub frames: &'static [&'static str],
    pub length: usize,
}

#[derive(Clone)]
pub struct AnimationState {
    pub loading_spinner: Animation,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            loading_spinner: LOADING_SPINNER_ANIMATION,
        }
    }
}

#[derive(Clone)]
pub struct Animation {
    frame_set: FrameSet,
    frame: usize,
    pub frame_height: u8,
}

impl Animation {
    pub fn new(frame_set: FrameSet, frame_height: u8) -> Self {
        Self {
            frame_set,
            frame: 0,
            frame_height,
        }
    }
}

impl Animation {
    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % self.frame_set.length;
    }

    pub fn render(&self) -> Vec<Line<'_>> {
        self.frame_set.frames[self.frame]
            .lines()
            .map(|line| Line::from(line.to_string()))
            .collect::<Vec<_>>()
    }
}
