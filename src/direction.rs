// TODO(thlorenz): move to separate module and impl From<Rad> where struct Rad(f32);
#[derive(Debug, PartialEq)]
pub(crate) enum DirectionX {
    Left,
    Right,
    Parallel,
}

#[derive(Debug, PartialEq)]
pub(crate) enum DirectionY {
    Up,
    Down,
    Parallel,
}
