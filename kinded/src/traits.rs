use ::core::fmt::Debug;

/// A trait that can be implemented by a main enum type.
/// Typically should be derived with `#[derive(kinded::Kinded)]`.
pub trait Kinded {
    type Kind: PartialEq + Eq + Debug + Clone + Copy + Kind;

    /// Get a kind variant without data.
    fn kind(&self) -> Self::Kind;
}

pub trait Kind: PartialEq + Eq + Debug + Clone + Copy {
    /// Return a vector with all possible kind variants.
    fn all() -> Vec<Self>;
}
