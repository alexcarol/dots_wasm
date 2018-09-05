/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool,
    pub click: (f64, f64),
    pub mouse_position: (f64, f64),
    pub mouseup: bool,
}
