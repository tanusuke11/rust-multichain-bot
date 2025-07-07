pub struct VPSEnvironment;
impl crate::env::Environment for VPSEnvironment {
    fn setup(&self) {
        println!("Setting up VPS Environment");
    }
}
