## Unreleased
- Add `attrs` attribute to pass extra attributes to the generated kind enum (e.g., `#[kinded(attrs(serde(rename_all = "snake_case")))]`)
- Add per-variant `attrs` attribute to pass attributes to individual kind variants (e.g., `#[kinded(attrs(default))]`) (fixes #22)
- Make `kind()` method `const fn`, allowing usage in const contexts (fixes #12)

## v0.4.1 - 2026-01-29
- Add `#[kinded(rename = "...")]` attribute for variants to customize display/parse names.
  This is useful when the automatic case conversion doesn't produce the desired result.

## v0.4.0 - 2025-10-30
- Update to Rust 2024 edition.
- no_std support.

## v0.3.0 - 2023-08-09
- Make `::all()` function return an array instead of vector.

## v0.2.0 - 2023-08-06
- Add `Kind` trait.

## v0.1.1 - 2023-08-06
- Add `::all()` to the kind type to iterate over all kind variants
- Generate customizable implementation of `Display` trait
- Generate implementation of `FromStr` trait

## v0.0.3 - 2023-08-05
- Make generated `kind()` function public

## v0.0.2 - 2023-08-05
- Support enums with generics

## v0.0.1 - 2023-08-04
- Very initial release
