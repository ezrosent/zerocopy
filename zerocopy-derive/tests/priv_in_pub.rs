// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

use zerocopy::{FromBytes, FromZeros, IntoBytes, KnownLayout, Unaligned};

// These derives do not result in E0446 as of Rust 1.59.0, because of
// https://github.com/rust-lang/rust/pull/90586.
//
// This change eliminates one of the major downsides of emitting `where`
// bounds for field types (i.e., the emission of E0446 for private field
// types).

#[derive(KnownLayout, IntoBytes, FromZeros, FromBytes, Unaligned)]
#[repr(C)]
pub struct Public(Private);

#[derive(KnownLayout, IntoBytes, FromZeros, FromBytes, Unaligned)]
#[repr(C)]
struct Private(());
