// Environment switching logic and common interface

pub trait Environment {
    fn setup(&self);
}

pub struct LocalEnvironment;
impl LocalEnvironment {
    pub fn new() -> Self {
        LocalEnvironment
    }
}
impl Environment for LocalEnvironment {
    fn setup(&self) {
        println!("Setting up Local Environment with local configurations...");
    }
}

pub struct RemoteEnvironment;
impl RemoteEnvironment {
    pub fn new() -> Self {
        RemoteEnvironment
    }
}
impl Environment for RemoteEnvironment {
    fn setup(&self) {
        println!("Setting up Remote Environment with cloud configurations...");
    }
}
