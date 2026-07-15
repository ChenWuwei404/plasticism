pub struct Radii {
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
    pub top_left: f64,
}
impl Radii {
    pub fn new(top_right: f64, bottom_right: f64, bottom_left: f64, top_left: f64) -> Self {
        Radii { top_right, bottom_right, bottom_left, top_left }
    }
    pub fn same(radius: f64) -> Self {
        Radii { top_right: radius, bottom_right: radius, bottom_left: radius, top_left: radius }
    }
}