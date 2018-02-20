pub mod card;
pub mod dice;

trait BetSerde: Sized {
    fn from_u16(id: u16) -> Option<Self>;
    fn to_u16(&self) -> u16;
}
