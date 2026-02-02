//! This example demonstrates how to use `kinded` with `enumset`.
//!
//! The problem: `enumset`'s `EnumSetType` only works on fieldless enums,
//! but you may have an enum with associated data that you want to use with EnumSet.
//!
//! The solution: Use `kinded` to generate a fieldless "kind" enum, then derive
//! `EnumSetType` on that kind enum.
//!
//! The catch: Both `kinded` and `enumset` implement `Copy`, `Clone`, `PartialEq`, `Eq`
//! by default, which causes trait implementation conflicts.
//!
//! The fix: Use `skip_derive` to prevent `kinded` from implementing those traits,
//! letting `enumset` handle them instead.

use enumset::{EnumSet, EnumSetType};
use kinded::Kinded;

/// A permission with associated data - cannot directly use EnumSetType.
///
/// We use kinded to generate `PermissionKind` (a fieldless enum), and:
/// - `skip_derive(Clone, Copy, PartialEq, Eq)` to avoid conflicts with enumset
/// - `derive(EnumSetType)` to make the kind enum work with EnumSet
/// - `attrs(enumset(...), repr(u8))` to configure enumset
#[derive(Kinded)]
#[kinded(
    skip_derive(Clone, Copy, PartialEq, Eq),
    derive(EnumSetType),
    attrs(enumset(repr = "u8"), repr(u8))
)]
enum Permission {
    Read { path: String },
    Write { path: String },
    Execute { command: String },
    Admin,
}

/// Check if a permission is allowed by a permission set, and return a description.
fn check_permission(perm: &Permission, allowed: EnumSet<PermissionKind>) -> String {
    if allowed.contains(perm.kind()) {
        match perm {
            Permission::Read { path } => format!("Allowed to read: {path}"),
            Permission::Write { path } => format!("Allowed to write: {path}"),
            Permission::Execute { command } => format!("Allowed to execute: {command}"),
            Permission::Admin => "Admin access granted".to_owned(),
        }
    } else {
        format!("Permission denied: {:?}", perm.kind())
    }
}

fn main() {
    // Create permissions with associated data
    let read_home = Permission::Read {
        path: "/home".to_owned(),
    };
    let write_tmp = Permission::Write {
        path: "/tmp".to_owned(),
    };
    let exec_ls = Permission::Execute {
        command: "ls".to_owned(),
    };
    let admin = Permission::Admin;

    // Extract kinds from permissions
    assert_eq!(read_home.kind(), PermissionKind::Read);
    assert_eq!(write_tmp.kind(), PermissionKind::Write);
    assert_eq!(exec_ls.kind(), PermissionKind::Execute);
    assert_eq!(admin.kind(), PermissionKind::Admin);

    // Now use EnumSet with the kind enum
    let user_permissions: EnumSet<PermissionKind> =
        PermissionKind::Read | PermissionKind::Write;

    let admin_permissions: EnumSet<PermissionKind> = EnumSet::all();

    // Check if a permission kind is in the set
    assert!(user_permissions.contains(PermissionKind::Read));
    assert!(user_permissions.contains(PermissionKind::Write));
    assert!(!user_permissions.contains(PermissionKind::Admin));

    // Admin has all permissions
    assert!(admin_permissions.contains(PermissionKind::Admin));
    assert!(admin_permissions.contains(PermissionKind::Execute));

    // Check if a specific permission instance is allowed
    assert!(user_permissions.contains(read_home.kind()));
    assert!(user_permissions.contains(write_tmp.kind()));
    assert!(!user_permissions.contains(exec_ls.kind()));

    // Set operations
    let execute_only: EnumSet<PermissionKind> = EnumSet::only(PermissionKind::Execute);
    let combined = user_permissions | execute_only;
    assert_eq!(combined.len(), 3);
    assert!(combined.contains(PermissionKind::Read));
    assert!(combined.contains(PermissionKind::Write));
    assert!(combined.contains(PermissionKind::Execute));

    // Iterate over permission kinds in a set
    let mut count = 0;
    for _kind in user_permissions {
        count += 1;
    }
    assert_eq!(count, 2);

    // Use the helper function to check permissions with their data
    assert_eq!(
        check_permission(&read_home, user_permissions),
        "Allowed to read: /home"
    );
    assert_eq!(
        check_permission(&write_tmp, user_permissions),
        "Allowed to write: /tmp"
    );
    assert_eq!(
        check_permission(&exec_ls, user_permissions),
        "Permission denied: Execute"
    );
    assert_eq!(
        check_permission(&admin, admin_permissions),
        "Admin access granted"
    );
}
