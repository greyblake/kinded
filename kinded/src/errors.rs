/// An error which is returned when parsing of a kind type failures.
#[derive(Debug)]
pub struct ParseKindError {
    kind_type_name: String,
    given_string: String,
}

impl ParseKindError {
    /// This method is used by `kinded` macro to construct an error for FromStr trait and is not
    /// recommend for a direct usage by users.
    pub fn from_type_name_and_string(
        kind_type_name: String,
        given_string: String,
    ) -> ParseKindError {
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
        write!(
            f,
            r#"Failed to parse {kind_type_name} from "{given_string}""#
        )
    }
}

impl ::std::error::Error for ParseKindError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        None
    }
}
