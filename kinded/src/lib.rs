pub use kinded_macros::Kinded;
use std::fmt::Debug;

pub trait Kinded {
    type Kind: PartialEq + Eq + Debug + Clone + Copy;

    fn kind(&self) -> Self::Kind;
}
