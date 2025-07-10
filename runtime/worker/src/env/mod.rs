pub mod local;
pub mod vps;

pub use local::LocalEnvironment;
pub use vps::VPSEnvironment;

pub trait Environment {
    fn setup(&self);
}