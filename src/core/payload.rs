#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Padding {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}
impl Padding {
    fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Padding { top, right, bottom, left }
    }
    fn same(space: i32) -> Self {
        Padding { top: space, right: space, bottom: space, left: space }
    }
    fn symmetric(vertical: i32, horizontal: i32) -> Self {
        Padding { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Margin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}
impl Margin {
    fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Margin { top, right, bottom, left }
    }
    fn same(space: i32) -> Self {
        Margin { top: space, right: space, bottom: space, left: space }
    }
    fn symmetric(vertical: i32, horizontal: i32) -> Self {
        Margin { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
}
