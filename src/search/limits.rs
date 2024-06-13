pub struct SearchLimits {
    pub depth: Option<u64>,
    pub nodes: Option<u64>,
    pub movetime: Option<u64>,
    pub white_time: Option<u64>,
    pub black_time: Option<u64>,
    pub white_increment: Option<u64>,
    pub black_increment: Option<u64>,
}

impl Default for SearchLimits {
    fn default() -> Self {
        SearchLimits::new()
    }
}

impl SearchLimits {
    pub fn new() -> Self {
        SearchLimits {
            depth: None,
            nodes: None,
            movetime: None,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
        }
    }

    pub fn depth(mut self, depth: Option<u64>) -> Self {
        self.depth = depth;
        self
    }

    pub fn movetime(mut self, movetime: Option<u64>) -> Self {
        self.movetime = movetime;
        self
    }

    pub fn white_time(mut self, white_time: Option<u64>) -> Self {
        self.white_time = white_time;
        self
    }

    pub fn black_time(mut self, black_time: Option<u64>) -> Self {
        self.black_time = black_time;
        self
    }

    pub fn nodes(mut self, nodes: Option<u64>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn white_increment(mut self, white_increment: Option<u64>) -> Self {
        self.white_increment = white_increment;
        self
    }

    pub fn black_increment(mut self, black_increment: Option<u64>) -> Self {
        self.black_increment = black_increment;
        self
    }
}
