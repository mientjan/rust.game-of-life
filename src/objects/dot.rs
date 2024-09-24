#[derive(Clone)]
pub struct Dot {
    pub(crate) index: u32,
    pub(crate) state: bool,
}

impl Dot {
    pub fn new(index: u32, state: bool) -> Self {
        Dot { index, state }
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    pub fn clone(&self) -> Dot {
        Dot { index: self.index, state: self.state }
    }
}