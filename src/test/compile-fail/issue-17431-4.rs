// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker;

struct Foo<T> { foo: Option<Option<Foo<T>>>, marker: marker::PhantomData<T> }
//~^ ERROR illegal recursive struct type; wrap the inner value in a box to make it representable

impl<T> Foo<T> { fn bar(&self) {} }

fn main() {}
