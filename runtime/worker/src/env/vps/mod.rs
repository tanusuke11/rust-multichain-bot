pub struct VPSEnvironment;

impl VPSEnvironment {
    pub fn new() -> Self {
        Self
    }
}

impl crate::env::Environment for VPSEnvironment {
    fn setup(&self) {
        println!("Setting up VPS Environment");
    }
}
