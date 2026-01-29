#![allow(unused_imports)]
#![allow(dead_code)]
#![no_std]

extern crate alloc;

use kinded::Kinded;

#[derive(Kinded)]
enum Role {
    Guest,
    User(i32),
    #[allow(dead_code)]
    Admin {
        id: i32,
    },
}

mod main_enum {
    use super::*;

    mod fn_kind {
        use super::*;

        #[test]
        fn should_convert_unit_variant() {
            let guest = Role::Guest;
            assert_eq!(guest.kind(), RoleKind::Guest);
        }

        #[test]
        fn should_convert_unnamed_variant() {
            let user = Role::User(13);
            assert_eq!(user.kind(), RoleKind::User);
        }

        #[test]
        fn should_convert_named_variant() {
            let admin = Role::Admin { id: 404 };
            assert_eq!(admin.kind(), RoleKind::Admin);
        }
    }

    mod traits {
        use super::*;
        use kinded::Kinded;

        fn compute_kind<T: Kinded>(val: T) -> <T as Kinded>::Kind {
            val.kind()
        }

        #[test]
        fn should_implement_kinded() {
            let admin = Role::Admin { id: 32 };
            assert_eq!(compute_kind(admin), RoleKind::Admin);
        }
    }
}

mod kind_enum {
    use super::RoleKind;

    mod traits {
        extern crate alloc;
        use alloc::format;

        use super::super::{Role, RoleKind};

        #[test]
        fn should_implement_debug() {
            assert_eq!(format!("{:?}", RoleKind::Guest), "Guest")
        }

        #[test]
        fn should_implement_clone() {
            let _ = RoleKind::Admin;
        }

        #[test]
        fn should_implement_copy() {
            fn receive_copy<T: Copy>() {}

            receive_copy::<RoleKind>();
        }

        #[test]
        fn should_implement_eq() {
            assert!(RoleKind::Guest.eq(&RoleKind::Guest));
            assert!(!RoleKind::Guest.eq(&RoleKind::User));
        }

        #[test]
        fn should_implement_from() {
            let user = Role::User(123);
            assert_eq!(RoleKind::from(user), RoleKind::User);
        }

        #[test]
        fn should_implement_from_ref() {
            let guest = Role::Guest;
            assert_eq!(RoleKind::from(&guest), RoleKind::Guest);
        }

        mod display_trait {
            extern crate alloc;
            use alloc::{format, string::ToString};

            use super::RoleKind;

            #[test]
            fn should_implement_display() {
                let guest = RoleKind::Guest;
                assert_eq!(format!("{guest}"), "Guest");

                let user = RoleKind::User;
                assert_eq!(format!("{user}"), "User");
            }

            #[test]
            fn should_display_snake_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "snake_case")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "hot_mate")
            }

            #[test]
            fn should_display_camel_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "camelCase")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "hotMate")
            }

            #[test]
            fn should_display_pascal_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "PascalCase")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "HotMate")
            }

            #[test]
            fn should_display_screaming_snake_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "SCREAMING_SNAKE_CASE")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "HOT_MATE")
            }

            #[test]
            fn should_display_kebab_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "kebab-case")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "hot-mate")
            }

            #[test]
            fn should_display_screaming_kebab_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "SCREAMING-KEBAB-CASE")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "HOT-MATE")
            }

            #[test]
            fn should_display_title_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "Title Case")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "Hot Mate")
            }

            #[test]
            fn should_display_lower_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "lowercase")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "hotmate")
            }

            #[test]
            fn should_display_upper_case() {
                #[derive(kinded::Kinded)]
                #[kinded(display = "UPPERCASE")]
                enum Drink {
                    HotMate,
                }

                assert_eq!(DrinkKind::HotMate.to_string(), "HOTMATE")
            }
        }

        mod from_str_trait {
            extern crate alloc;
            use alloc::string::ToString;

            #[derive(kinded::Kinded)]
            enum Mate {
                HotMate,
                Terere,
            }

            #[test]
            fn should_implement_from_str_trait() {
                let kind: MateKind = "Terere".parse().unwrap();
                assert_eq!(kind, MateKind::Terere);

                let kind: MateKind = "HotMate".parse().unwrap();
                assert_eq!(kind, MateKind::HotMate);
            }

            #[test]
            fn should_parse_alternative_cases() {
                // All possible alternatives of HoteMate
                let hot_mate_alternatives = [
                    "hot_mate", // snake_case
                    "hotMate",  // camelCase
                    "HotMate",  // PascalCase
                    "HOT_MATE", // SCREAMING_SNAKE_CASE
                    "hot-mate", // kebab-case
                    "HOT-MATE", // SCREAMING-KEBAB-CASE
                    "Hot Mate", // Title Case
                    "hotmate",  // lowercase
                    "HOTMATE",  // UPPERCASE
                ];
                for alt in hot_mate_alternatives {
                    let kind: MateKind = alt.parse().unwrap();
                    assert_eq!(kind, MateKind::HotMate);
                }

                // Just a few alternatives of Terere
                let terere_alternatives = ["terere", "TERERE", "Terere"];
                for alt in terere_alternatives {
                    let kind: MateKind = alt.parse().unwrap();
                    assert_eq!(kind, MateKind::Terere);
                }
            }

            #[test]
            fn should_return_error_on_failure() {
                let error: kinded::ParseKindError = "Calabaza".parse::<MateKind>().unwrap_err();
                assert_eq!(
                    error.to_string(),
                    r#"Failed to parse "Calabaza" as MateKind"#
                );
            }

            #[test]
            fn should_distinguish_very_similar_abbreviations() {
                #[derive(kinded::Kinded)]
                enum Db {
                    MySql,
                    MySQL,
                }

                assert_eq!("MySql".parse::<DbKind>().unwrap(), DbKind::MySql);
                assert_eq!("MySQL".parse::<DbKind>().unwrap(), DbKind::MySQL);
            }
        }

        mod kind_trait {
            use crate::RoleKind;

            #[test]
            fn should_implement_kind_trait() {
                assert_eq!(
                    RoleKind::all(),
                    [RoleKind::Guest, RoleKind::User, RoleKind::Admin]
                )
            }
        }
    }

    #[test]
    fn should_provide_all_function_that_returns_iterator() {
        fn impl_iter(_: impl IntoIterator<Item = &'static RoleKind>) {}
        impl_iter(RoleKind::all());
    }
}

#[test]
fn should_allow_to_give_custom_name_kind_type() {
    #[derive(Kinded)]
    #[kinded(kind = SimpleDrink)]
    enum Drink {
        Tea(&'static str),
        Coffee(&'static str),
    }

    let green_tea = Drink::Tea("Green");
    assert_eq!(green_tea.kind(), SimpleDrink::Tea);
}

#[test]
fn should_allow_to_derive_custom_traits() {
    #[derive(Kinded)]
    #[kinded(derive(Hash, Eq, PartialOrd, Ord))]
    enum Drink {
        Tea(&'static str),
        Coffee(&'static str),
    }

    let mut drinks = alloc::collections::BTreeMap::new();
    drinks.insert(DrinkKind::Tea, 5);
}

#[test]
fn should_work_with_generics() {
    #[derive(Kinded)]
    enum Maybe<T> {
        Just(T),
        Nothing,
    }

    assert_eq!(Maybe::Just(13).kind(), MaybeKind::Just);
}

#[test]
fn should_work_with_lifetimes() {
    #[derive(Kinded)]
    enum Identifier<'a, I> {
        Name(&'a str),
        Id(I),
    }

    let identifier: Identifier<i32> = Identifier::Name("Xen");
    assert_eq!(identifier.kind(), IdentifierKind::Name);
}

mod rename {
    extern crate alloc;
    use alloc::string::ToString;
    use kinded::Kinded;

    /// Test that rename overrides the Display output
    #[test]
    fn should_display_renamed_variant() {
        #[derive(Kinded)]
        enum Validator {
            NotEmpty,
            #[kinded(rename = "len_utf16_min")]
            LenUtf16Min,
        }

        assert_eq!(ValidatorKind::NotEmpty.to_string(), "NotEmpty");
        assert_eq!(ValidatorKind::LenUtf16Min.to_string(), "len_utf16_min");
    }

    /// Test that rename overrides the automatic case conversion
    #[test]
    fn should_override_display_case_with_rename() {
        #[derive(Kinded)]
        #[kinded(display = "snake_case")]
        enum Validator {
            NotEmpty,
            // Without rename, this would display as "len_utf_16_min" (with extra underscore)
            #[kinded(rename = "len_utf16_min")]
            LenUtf16Min,
        }

        assert_eq!(ValidatorKind::NotEmpty.to_string(), "not_empty");
        assert_eq!(ValidatorKind::LenUtf16Min.to_string(), "len_utf16_min");
    }

    /// Test that FromStr parses the renamed value
    #[test]
    fn should_parse_renamed_value() {
        #[derive(Kinded)]
        #[kinded(display = "snake_case")]
        enum Validator {
            NotEmpty,
            #[kinded(rename = "len_utf16_min")]
            LenUtf16Min,
        }

        // Parse the renamed value
        let kind: ValidatorKind = "len_utf16_min".parse().unwrap();
        assert_eq!(kind, ValidatorKind::LenUtf16Min);
    }

    /// Test that original variant name and alternatives still parse correctly
    #[test]
    fn should_still_parse_original_names() {
        #[derive(Kinded)]
        #[kinded(display = "snake_case")]
        enum Validator {
            NotEmpty,
            #[kinded(rename = "len_utf16_min")]
            LenUtf16Min,
        }

        // Original name should still work
        assert_eq!(
            "LenUtf16Min".parse::<ValidatorKind>().unwrap(),
            ValidatorKind::LenUtf16Min
        );

        // Alternative cases should also work
        assert_eq!(
            "len_utf_16_min".parse::<ValidatorKind>().unwrap(),
            ValidatorKind::LenUtf16Min
        );
        assert_eq!(
            "LEN_UTF_16_MIN".parse::<ValidatorKind>().unwrap(),
            ValidatorKind::LenUtf16Min
        );
    }

    /// Test rename with multiple renamed variants
    #[test]
    fn should_work_with_multiple_renames() {
        #[derive(Kinded)]
        #[kinded(display = "snake_case")]
        enum Validator {
            #[kinded(rename = "len_utf16_min")]
            LenUtf16Min,
            #[kinded(rename = "len_utf16_max")]
            LenUtf16Max,
            NotEmpty,
        }

        assert_eq!(ValidatorKind::LenUtf16Min.to_string(), "len_utf16_min");
        assert_eq!(ValidatorKind::LenUtf16Max.to_string(), "len_utf16_max");
        assert_eq!(ValidatorKind::NotEmpty.to_string(), "not_empty");

        assert_eq!(
            "len_utf16_min".parse::<ValidatorKind>().unwrap(),
            ValidatorKind::LenUtf16Min
        );
        assert_eq!(
            "len_utf16_max".parse::<ValidatorKind>().unwrap(),
            ValidatorKind::LenUtf16Max
        );
    }

    /// Test rename with variants that have data
    #[test]
    fn should_work_with_data_variants() {
        #[derive(Kinded)]
        enum Action {
            #[kinded(rename = "custom_action")]
            DoSomething(i32),
            #[kinded(rename = "other")]
            DoOther { value: i32 },
            Plain,
        }

        assert_eq!(ActionKind::DoSomething.to_string(), "custom_action");
        assert_eq!(ActionKind::DoOther.to_string(), "other");
        assert_eq!(ActionKind::Plain.to_string(), "Plain");

        assert_eq!(
            "custom_action".parse::<ActionKind>().unwrap(),
            ActionKind::DoSomething
        );
        assert_eq!("other".parse::<ActionKind>().unwrap(), ActionKind::DoOther);
    }
}
