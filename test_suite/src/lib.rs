#![allow(unused_imports)]
#![allow(dead_code)]

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

mod base_enum {
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
        use super::super::{Role, RoleKind};

        #[test]
        fn should_implement_debug() {
            assert_eq!(format!("{:?}", RoleKind::Guest), "Guest")
        }

        #[test]
        fn should_implement_clone() {
            let _ = RoleKind::Admin.clone();
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
    }

    #[test]
    fn should_provide_all_function_that_returns_iterator() {
        let kinds: Vec<_> = RoleKind::all().collect();
        assert_eq!(
            kinds,
            vec![RoleKind::Guest, RoleKind::User, RoleKind::Admin],
        )
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
    use std::collections::HashMap;

    #[derive(Kinded)]
    #[kinded(derive(Hash, Eq))]
    enum Drink {
        Tea(&'static str),
        Coffee(&'static str),
    }

    let mut drinks = HashMap::new();
    drinks.insert(DrinkKind::Tea, 5);
}

#[test]
fn should_work_with_generics() {
    use std::collections::HashMap;

    #[derive(Kinded)]
    enum Maybe<T> {
        Just(T),
        Nothing,
    }

    assert_eq!(Maybe::Just(13).kind(), MaybeKind::Just);
}

#[test]
fn should_work_with_lifetimes() {
    use std::collections::HashMap;

    #[derive(Kinded)]
    enum Identifier<'a, I> {
        Name(&'a str),
        Id(I),
    }

    let identifier: Identifier<i32> = Identifier::Name("Xen");
    assert_eq!(identifier.kind(), IdentifierKind::Name);
}
