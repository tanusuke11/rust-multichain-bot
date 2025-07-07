pub trait Environment {
    fn setup(&self);
}

pub struct LocalEnvironment;
impl Environment for LocalEnvironment {
    fn setup(&self) {
        println!("Setting up Local Environment");
    }
}

pub struct VPSEnvironment;
impl Environment for VPSEnvironment {
    fn setup(&self) {
        println!("Setting up VPS Environment");
    }
}
