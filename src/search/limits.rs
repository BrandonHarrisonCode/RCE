use super::{Depth, Millisecond, NodeCount};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchLimits {
    pub perft: bool,
    pub depth: Option<Depth>,
    pub nodes: Option<NodeCount>,
    pub movetime: Option<Millisecond>,
    pub white_time: Option<Millisecond>,
    pub black_time: Option<Millisecond>,
    pub white_increment: Option<Millisecond>,
    pub black_increment: Option<Millisecond>,

    pub time_management_timer: Option<Millisecond>,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchLimits {
    pub const fn new() -> Self {
        Self {
            perft: false,
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

    pub const fn depth(mut self, depth: Option<Depth>) -> Self {
        self.depth = depth;
        self
    }

    pub const fn movetime(mut self, movetime: Option<Millisecond>) -> Self {
        self.movetime = movetime;
        self
    }

    pub const fn white_time(mut self, white_time: Option<Millisecond>) -> Self {
        self.white_time = white_time;
        self
    }

    pub const fn black_time(mut self, black_time: Option<Millisecond>) -> Self {
        self.black_time = black_time;
        self
    }

    pub const fn nodes(mut self, nodes: Option<NodeCount>) -> Self {
        self.nodes = nodes;
        self
    }

    pub const fn white_increment(mut self, white_increment: Option<Millisecond>) -> Self {
        self.white_increment = white_increment;
        self
    }

    pub const fn black_increment(mut self, black_increment: Option<Millisecond>) -> Self {
        self.black_increment = black_increment;
        self
    }

    #[allow(dead_code)]
    pub const fn time_management_timer(
        mut self,
        time_management_timer: Option<Millisecond>,
    ) -> Self {
        self.time_management_timer = time_management_timer;
        self
    }
}
