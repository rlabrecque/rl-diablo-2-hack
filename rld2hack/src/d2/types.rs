pub struct AutomapOffset {
    x: i32,
    y: i32,
}

impl std::fmt::Display for AutomapOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Normal = 0x00,
    Nightmare = 0x01,
    Hell = 0x02,
}
