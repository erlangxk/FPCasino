pub mod sicbo;
pub mod roulette;

pub trait Ratio {
    fn ratio(&self)->f64;
}
pub trait BetId{
    fn id(&self)->u16;
}
