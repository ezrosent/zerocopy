// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

#![allow(warnings)]

mod util;

use std::{marker::PhantomData, mem::ManuallyDrop, option::IntoIter};

use {static_assertions::assert_impl_all, zerocopy::IntoBytes};

use self::util::AU16;

// A struct is `IntoBytes` if:
// - all fields are `IntoBytes`
// - `repr(C)` or `repr(transparent)` and
//   - no padding (size of struct equals sum of size of field types)
// - `repr(packed)`

#[derive(IntoBytes)]
#[repr(C)]
struct CZst;

assert_impl_all!(CZst: IntoBytes);

#[derive(IntoBytes)]
#[repr(C)]
struct C {
    a: u8,
    b: u8,
    c: AU16,
}

assert_impl_all!(C: IntoBytes);

#[derive(IntoBytes)]
#[repr(transparent)]
struct Transparent {
    a: u8,
    b: CZst,
}

assert_impl_all!(Transparent: IntoBytes);

#[derive(IntoBytes)]
#[repr(transparent)]
struct TransparentGeneric<T: ?Sized> {
    a: CZst,
    b: T,
}

assert_impl_all!(TransparentGeneric<u64>: IntoBytes);
assert_impl_all!(TransparentGeneric<[u64]>: IntoBytes);

#[derive(IntoBytes)]
#[repr(C, packed)]
struct CZstPacked;

assert_impl_all!(CZstPacked: IntoBytes);

#[derive(IntoBytes)]
#[repr(C, packed)]
struct CPacked {
    a: u8,
    // NOTE: The `u16` type is not guaranteed to have alignment 2, although it
    // does on many platforms. However, to fix this would require a custom type
    // with a `#[repr(align(2))]` attribute, and `#[repr(packed)]` types are not
    // allowed to transitively contain `#[repr(align(...))]` types. Thus, we
    // have no choice but to use `u16` here. Luckily, these tests run in CI on
    // platforms on which `u16` has alignment 2, so this isn't that big of a
    // deal.
    b: u16,
}

assert_impl_all!(CPacked: IntoBytes);

#[derive(IntoBytes)]
#[repr(C, packed(2))]
// The same caveats as for CPacked apply - we're assuming u64 is at least
// 4-byte aligned by default. Without packed(2), this should fail, as there
// would be padding between a/b assuming u64 is 4+ byte aligned.
struct CPacked2 {
    a: u16,
    b: u64,
}

assert_impl_all!(CPacked2: IntoBytes);

#[derive(IntoBytes)]
#[repr(C, packed)]
struct CPackedGeneric<T, U: ?Sized> {
    t: T,
    // Unsized types stored in `repr(packed)` structs must not be dropped
    // because dropping them in-place might be unsound depending on the
    // alignment of the outer struct. Sized types can be dropped by first being
    // moved to an aligned stack variable, but this isn't possible with unsized
    // types.
    u: ManuallyDrop<U>,
}

assert_impl_all!(CPackedGeneric<u8, AU16>: IntoBytes);
assert_impl_all!(CPackedGeneric<u8, [AU16]>: IntoBytes);

#[derive(IntoBytes)]
#[repr(packed)]
struct Packed {
    a: u8,
    // NOTE: The `u16` type is not guaranteed to have alignment 2, although it
    // does on many platforms. However, to fix this would require a custom type
    // with a `#[repr(align(2))]` attribute, and `#[repr(packed)]` types are not
    // allowed to transitively contain `#[repr(align(...))]` types. Thus, we
    // have no choice but to use `u16` here. Luckily, these tests run in CI on
    // platforms on which `u16` has alignment 2, so this isn't that big of a
    // deal.
    b: u16,
}

assert_impl_all!(Packed: IntoBytes);

#[derive(IntoBytes)]
#[repr(packed)]
struct PackedGeneric<T, U: ?Sized> {
    t: T,
    // Unsized types stored in `repr(packed)` structs must not be dropped
    // because dropping them in-place might be unsound depending on the
    // alignment of the outer struct. Sized types can be dropped by first being
    // moved to an aligned stack variable, but this isn't possible with unsized
    // types.
    u: ManuallyDrop<U>,
}

assert_impl_all!(PackedGeneric<u8, AU16>: IntoBytes);
assert_impl_all!(PackedGeneric<u8, [AU16]>: IntoBytes);

#[derive(IntoBytes)]
#[repr(transparent)]
struct Unsized {
    a: [u8],
}

assert_impl_all!(Unsized: IntoBytes);

// Deriving `IntoBytes` should work if the struct has bounded parameters.

#[derive(IntoBytes)]
#[repr(transparent)]
struct WithParams<'a: 'b, 'b: 'a, T: 'a + 'b + IntoBytes, const N: usize>(
    [T; N],
    PhantomData<&'a &'b ()>,
)
where
    'a: 'b,
    'b: 'a,
    T: 'a + 'b + IntoBytes;

assert_impl_all!(WithParams<'static, 'static, u8, 42>: IntoBytes);
