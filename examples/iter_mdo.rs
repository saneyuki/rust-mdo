// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(plugin, link)]
extern crate mdo;

fn main() {
    // exporting the monadic functions for the Iterator monad (similar
    // to list comprehension)
    use mdo::iter::{bind, ret, mzero};

    // getting the list of (x, y, z) such that
    //  - 1 <= x <= y < z < 11
    //  - x^2 + y^2 == z^2
    let l = bind(
        range(1i, 11), move |z|
        bind(range(1, z), move |x|
        bind(range(x, z), move |y|
        bind(if x * x + y * y == z * z { ret(()) } else { mzero() }, move |_|
        ret((x, y, z))
    )))).collect::<Vec<(int, int, int)>>();
    println!("{}", l);

    // the same thing, using the mdo! macro
    let l = mdo! {
        z <- range(1i, 11);
        x <- range(1, z);
        y <- range(x, z);
        when x * x + y * y == z * z;
        ret ret((x, y, z))
    }.collect::<Vec<(int, int, int)>>();
    println!("{}", l);
}
