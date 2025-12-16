// trait Draw containing the draw() signature.
pub trait Draw {
    fn draw(&self);
}

struct Screen {
    // we would want the users to manage their own components
    pub components: Vec<Box<dyn Draw>> // creating the trai object {vector defined by trait not by type}
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// note: we can't use generic type because then components would require concrete type but we want to 
// store different types that are united by a common trait.
