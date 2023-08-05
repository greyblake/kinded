use ::core::fmt::Debug;

/// A trait that can be implemented by a main enum type.
/// Typically should be derived with `#[derive(kinded::Kinded)]`.
pub trait Kinded {
    type Kind: PartialEq + Eq + Debug + Clone + Copy;

    fn kind(&self) -> Self::Kind;
}
