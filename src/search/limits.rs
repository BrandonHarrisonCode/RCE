#[allow(clippy::module_name_repetitions)]
pub struct SearchLimits {
    pub depth: Option<u128>,
    pub nodes: Option<u128>,
    pub movetime: Option<u128>,
    pub white_time: Option<u128>,
    pub black_time: Option<u128>,
    pub white_increment: Option<u128>,
    pub black_increment: Option<u128>,

    pub time_management_timer: Option<u128>,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchLimits {
    pub const fn new() -> Self {
        Self {
            depth: None,
            nodes: None,
            movetime: None,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,

            time_management_timer: None,
        }
    }

    pub const fn depth(mut self, depth: Option<u128>) -> Self {
        self.depth = depth;
        self
    }

    pub const fn movetime(mut self, movetime: Option<u128>) -> Self {
        self.movetime = movetime;
        self
    }

    pub const fn white_time(mut self, white_time: Option<u128>) -> Self {
        self.white_time = white_time;
        self
    }

    pub const fn black_time(mut self, black_time: Option<u128>) -> Self {
        self.black_time = black_time;
        self
    }

    pub const fn nodes(mut self, nodes: Option<u128>) -> Self {
        self.nodes = nodes;
        self
    }

    pub const fn white_increment(mut self, white_increment: Option<u128>) -> Self {
        self.white_increment = white_increment;
        self
    }

    pub const fn black_increment(mut self, black_increment: Option<u128>) -> Self {
        self.black_increment = black_increment;
        self
    }

    #[allow(dead_code)]
    pub const fn time_management_timer(mut self, time_management_timer: Option<u128>) -> Self {
        self.time_management_timer = time_management_timer;
        self
    }
}
