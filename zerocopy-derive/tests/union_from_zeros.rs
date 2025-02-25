// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

#![allow(warnings)]

#[macro_use]
mod util;

use std::{marker::PhantomData, option::IntoIter};

use {static_assertions::assert_impl_all, zerocopy::FromZeros};

// A union is `FromZeros` if:
// - all fields are `FromZeros`

#[derive(Clone, Copy, FromZeros)]
union Zst {
    a: (),
}

assert_impl_all!(Zst: FromZeros);

#[derive(FromZeros)]
union One {
    a: bool,
}

assert_impl_all!(One: FromZeros);

#[derive(FromZeros)]
union Two {
    a: bool,
    b: Zst,
}

assert_impl_all!(Two: FromZeros);

#[derive(FromZeros)]
union TypeParams<'a, T: Copy, I: Iterator>
where
    I::Item: Copy,
{
    a: T,
    c: I::Item,
    d: u8,
    e: PhantomData<&'a [u8]>,
    f: PhantomData<&'static str>,
    g: PhantomData<String>,
}

assert_impl_all!(TypeParams<'static, (), IntoIter<()>>: FromZeros);

// Deriving `FromZeros` should work if the union has bounded parameters.

#[derive(FromZeros)]
#[repr(C)]
union WithParams<'a: 'b, 'b: 'a, T: 'a + 'b + FromZeros, const N: usize>
where
    'a: 'b,
    'b: 'a,
    T: 'a + 'b + Copy + FromZeros,
{
    a: [T; N],
    b: PhantomData<&'a &'b ()>,
}

assert_impl_all!(WithParams<'static, 'static, u8, 42>: FromZeros);
