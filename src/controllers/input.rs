/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool,
    pub click: (i32, i32),
    pub mouse_position: (i32, i32),
    pub mouseup: bool,
}
