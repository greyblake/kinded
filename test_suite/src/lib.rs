#![allow(unused_imports)]
#![allow(dead_code)]
#![no_std]

extern crate alloc;

use kinded::Kinded;
use serde::{Deserialize, Serialize};

#[derive(Kinded, Serialize, Deserialize)]
#[kinded(derive(Serialize), attrs(serde(rename_all = "camelCase")))]
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

        mod attrs {
            use crate::RoleKind;
            use alloc::string::ToString;
            use serde::{Deserialize, Serialize};
            use serde_json::json;

            #[test]
            fn should_apply_single_attr() {
                // RoleKind has serde(rename_all = "camelCase") applied
                let value = serde_json::to_value(&RoleKind::Guest).unwrap();
                assert_eq!(value, json!("guest"));
            }

            #[test]
            fn should_apply_multiple_attrs() {
                #[derive(kinded::Kinded, Serialize, Deserialize)]
                #[kinded(
                    derive(Serialize, Deserialize),
                    attrs(
                        serde(rename_all = "snake_case"),
                        doc = "This is a generated kind enum"
                    )
                )]
                enum Vehicle {
                    SportsCar,
                    PickupTruck,
                    MotorCycle,
                }

                // Test serialization with snake_case
                let value = serde_json::to_value(&VehicleKind::SportsCar).unwrap();
                assert_eq!(value, json!("sports_car"));

                let value = serde_json::to_value(&VehicleKind::PickupTruck).unwrap();
                assert_eq!(value, json!("pickup_truck"));

                // Test deserialization
                let kind: VehicleKind = serde_json::from_str(r#""motor_cycle""#).unwrap();
                assert_eq!(kind, VehicleKind::MotorCycle);
            }

            #[test]
            fn should_work_with_name_value_attr() {
                #[derive(kinded::Kinded)]
                #[kinded(attrs(doc = "A beverage kind"))]
                enum Beverage {
                    Water,
                    Juice,
                }

                // If it compiles, the doc attribute was applied correctly
                let _ = BeverageKind::Water;
            }

            #[test]
            fn should_combine_with_kind_attr() {
                #[derive(kinded::Kinded, Serialize)]
                #[kinded(
                    kind = AnimalType,
                    derive(Serialize),
                    attrs(serde(rename_all = "SCREAMING_SNAKE_CASE"))
                )]
                enum Animal {
                    DomesticCat,
                    WildDog,
                }

                let value = serde_json::to_value(&AnimalType::DomesticCat).unwrap();
                assert_eq!(value, json!("DOMESTIC_CAT"));

                let value = serde_json::to_value(&AnimalType::WildDog).unwrap();
                assert_eq!(value, json!("WILD_DOG"));
            }

            #[test]
            fn should_combine_with_display_attr() {
                #[derive(kinded::Kinded, Serialize)]
                #[kinded(
                    display = "kebab-case",
                    derive(Serialize),
                    attrs(serde(rename_all = "kebab-case"))
                )]
                enum Fruit {
                    GreenApple,
                    RedCherry,
                }

                // Display should use kebab-case
                assert_eq!(FruitKind::GreenApple.to_string(), "green-apple");

                // Serde should also use kebab-case
                let value = serde_json::to_value(&FruitKind::RedCherry).unwrap();
                assert_eq!(value, json!("red-cherry"));
            }

            #[test]
            fn should_support_deserialization() {
                #[derive(kinded::Kinded, Serialize, Deserialize, PartialEq, Debug)]
                #[kinded(derive(Serialize, Deserialize), attrs(serde(rename_all = "camelCase")))]
                enum Planet {
                    MilkyWayEarth,
                    RedMars,
                }

                // Serialize
                let json = serde_json::to_string(&PlanetKind::MilkyWayEarth).unwrap();
                assert_eq!(json, r#""milkyWayEarth""#);

                // Deserialize
                let kind: PlanetKind = serde_json::from_str(r#""redMars""#).unwrap();
                assert_eq!(kind, PlanetKind::RedMars);

                // Round-trip
                let original = PlanetKind::MilkyWayEarth;
                let json = serde_json::to_string(&original).unwrap();
                let restored: PlanetKind = serde_json::from_str(&json).unwrap();
                assert_eq!(original, restored);
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

mod const_kind {
    use kinded::Kinded;

    #[derive(Kinded)]
    enum Status {
        Active,
        Inactive,
        Pending(i32),
        Custom { value: i32 },
    }

    /// Test that kind() can be used in const context with unit variant
    const ACTIVE_KIND: StatusKind = Status::Active.kind();

    /// Test that kind() can be used in const context with unit variant (another)
    const INACTIVE_KIND: StatusKind = Status::Inactive.kind();

    #[test]
    fn should_work_in_const_context_unit_variant() {
        assert_eq!(ACTIVE_KIND, StatusKind::Active);
        assert_eq!(INACTIVE_KIND, StatusKind::Inactive);
    }

    #[test]
    fn should_work_in_const_fn() {
        const fn get_kind(status: &Status) -> StatusKind {
            status.kind()
        }

        const STATUS: Status = Status::Active;
        const KIND: StatusKind = get_kind(&STATUS);
        assert_eq!(KIND, StatusKind::Active);
    }

    #[test]
    fn should_work_in_const_match() {
        const fn is_active(status: &Status) -> bool {
            matches!(status.kind(), StatusKind::Active)
        }

        const RESULT: bool = is_active(&Status::Active);
        assert!(RESULT);
    }

    /// Test with generic enum
    #[derive(Kinded)]
    enum Maybe<T> {
        Just(T),
        Nothing,
    }

    const NOTHING_KIND: MaybeKind = Maybe::<i32>::Nothing.kind();

    #[test]
    fn should_work_with_generic_enum_in_const() {
        assert_eq!(NOTHING_KIND, MaybeKind::Nothing);
    }

    /// Test with custom kind name
    #[derive(Kinded)]
    #[kinded(kind = SimpleStatus)]
    enum ComplexStatus {
        Ok,
        Error(i32),
    }

    const OK_KIND: SimpleStatus = ComplexStatus::Ok.kind();

    #[test]
    fn should_work_with_custom_kind_name_in_const() {
        assert_eq!(OK_KIND, SimpleStatus::Ok);
    }
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
            DoOther {
                value: i32,
            },
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

mod variant_attrs {
    extern crate alloc;
    use alloc::string::ToString;
    use kinded::Kinded;
    use serde::{Deserialize, Serialize};

    /// Test that a single attribute can be applied to a variant
    #[test]
    fn should_apply_single_attr_to_variant() {
        #[derive(Kinded)]
        #[kinded(derive(Default))]
        enum Priority {
            Low,
            #[kinded(attrs(default))]
            Medium,
            High,
        }

        assert_eq!(PriorityKind::default(), PriorityKind::Medium);
    }

    /// Test that multiple attributes can be applied to a variant
    #[test]
    fn should_apply_multiple_attrs_to_variant() {
        #[derive(Kinded, Serialize)]
        #[kinded(derive(Default, Serialize), attrs(serde(rename_all = "snake_case")))]
        enum Status {
            #[kinded(attrs(default, serde(rename = "waiting")))]
            Pending,
            Active,
            Done,
        }

        // Test default
        assert_eq!(StatusKind::default(), StatusKind::Pending);

        // Test serde rename on variant
        let json = serde_json::to_string(&StatusKind::Pending).unwrap();
        assert_eq!(json, r#""waiting""#);

        // Other variants should use enum-level rename_all
        let json = serde_json::to_string(&StatusKind::Active).unwrap();
        assert_eq!(json, r#""active""#);
    }

    /// Test attrs on multiple variants
    #[test]
    fn should_apply_attrs_to_multiple_variants() {
        #[derive(Kinded, Serialize)]
        #[kinded(derive(Serialize), attrs(serde(rename_all = "SCREAMING_SNAKE_CASE")))]
        enum Event {
            #[kinded(attrs(serde(rename = "UserLoggedIn")))]
            Login,
            #[kinded(attrs(serde(rename = "UserLoggedOut")))]
            Logout,
            // This one uses the enum-level rename_all
            SessionExpired,
        }

        assert_eq!(
            serde_json::to_string(&EventKind::Login).unwrap(),
            r#""UserLoggedIn""#
        );
        assert_eq!(
            serde_json::to_string(&EventKind::Logout).unwrap(),
            r#""UserLoggedOut""#
        );
        assert_eq!(
            serde_json::to_string(&EventKind::SessionExpired).unwrap(),
            r#""SESSION_EXPIRED""#
        );
    }

    /// Test combining variant attrs with rename
    #[test]
    fn should_combine_with_rename() {
        #[derive(Kinded, Serialize)]
        #[kinded(derive(Default, Serialize))]
        enum Level {
            #[kinded(rename = "low_level", attrs(default))]
            Low,
            Medium,
            High,
        }

        // Default should work
        assert_eq!(LevelKind::default(), LevelKind::Low);

        // Display should use rename
        assert_eq!(LevelKind::Low.to_string(), "low_level");
    }

    /// Test doc attribute on variants
    #[test]
    fn should_support_doc_attr() {
        #[derive(Kinded)]
        enum Color {
            #[kinded(attrs(doc = "The color red"))]
            Red,
            #[kinded(attrs(doc = "The color green"))]
            Green,
            Blue,
        }

        // If it compiles, the doc attributes were applied correctly
        let _ = ColorKind::Red;
        let _ = ColorKind::Green;
        let _ = ColorKind::Blue;
    }

    /// Test attrs on variants with data
    #[test]
    fn should_work_with_data_variants() {
        #[derive(Kinded)]
        #[kinded(derive(Default))]
        enum Container {
            #[kinded(attrs(default))]
            Empty,
            Single(i32),
            Multiple {
                items: i32,
            },
        }

        assert_eq!(ContainerKind::default(), ContainerKind::Empty);
    }

    /// Test deserialization with variant attrs
    #[test]
    fn should_support_deserialization_with_variant_attrs() {
        #[derive(Kinded, Serialize, Deserialize)]
        #[kinded(derive(Serialize, Deserialize))]
        enum Action {
            #[kinded(attrs(serde(rename = "create_new")))]
            Create,
            #[kinded(attrs(serde(rename = "read_existing")))]
            Read,
            Update,
            Delete,
        }

        // Serialize
        assert_eq!(
            serde_json::to_string(&ActionKind::Create).unwrap(),
            r#""create_new""#
        );

        // Deserialize
        let kind: ActionKind = serde_json::from_str(r#""create_new""#).unwrap();
        assert_eq!(kind, ActionKind::Create);

        let kind: ActionKind = serde_json::from_str(r#""read_existing""#).unwrap();
        assert_eq!(kind, ActionKind::Read);

        // Variants without attrs use default names
        let kind: ActionKind = serde_json::from_str(r#""Update""#).unwrap();
        assert_eq!(kind, ActionKind::Update);
    }
}
