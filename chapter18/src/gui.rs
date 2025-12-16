// trait Draw containing the draw() signature.
pub trait Draw {
    fn draw(&self);
}

struct Screen {
    // we would want the users to manage their own components
    pub components: Vec<Box<dyn Draw>> // creating the trai object {vector defined by trait not by type}
}


