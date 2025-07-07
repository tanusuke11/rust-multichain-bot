pub struct LocalEnvironment;
impl crate::env::Environment for LocalEnvironment {
    fn setup(&self) {
        println!("Setting up Local Environment");
    }
}
