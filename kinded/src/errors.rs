extern crate alloc;

use alloc::string::{String, ToString};

/// An error which is returned when parsing of a kind type failures.
pub struct ParseKindError {
    kind_type_name: String,
    given_string: String,
}

impl ParseKindError {
    /// This method is used by `kinded` macro to construct an error for FromStr trait and is not
    /// recommend for a direct usage by users.
    pub fn from_type_and_string<KindType>(given_string: String) -> ParseKindError {
        let full_kind_type_name = core::any::type_name::<KindType>();
        let kind_type_name = full_kind_type_name
            .split("::")
            .last()
            .expect("Type name cannot be empty")
            .to_string();
        ParseKindError {
            kind_type_name,
            given_string,
        }
    }
}

impl ::core::fmt::Display for ParseKindError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let Self {
            kind_type_name,
            given_string,
        } = self;
        write!(f, r#"Failed to parse "{given_string}" as {kind_type_name}"#)
    }
}

impl ::core::fmt::Debug for ParseKindError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
        write!(f, "ParseKindError: {self}")
    }
}
