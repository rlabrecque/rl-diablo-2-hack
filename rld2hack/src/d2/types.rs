pub struct AutomapOffset {
    x: i32,
    y: i32,
}

impl std::fmt::Display for AutomapOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
