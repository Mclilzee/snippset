pub struct Point {
    column: u16,
    row: u16,
}

pub struct CursorRange {
    current: Point,
    start: Point,
    end: Point,
}
