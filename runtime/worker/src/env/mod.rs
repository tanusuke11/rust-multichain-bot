pub mod local;
pub mod vps;

pub trait Environment {
    fn setup(&self);
}