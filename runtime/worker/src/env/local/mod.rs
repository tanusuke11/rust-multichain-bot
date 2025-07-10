pub struct LocalEnvironment;

impl LocalEnvironment {
    pub fn new() -> Self {
        Self
    }
}

impl crate::env::Environment for LocalEnvironment {
    fn setup(&self) {
        println!("Setting up Local Environment");
    }
}
