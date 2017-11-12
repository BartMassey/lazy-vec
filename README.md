# LazyVec
Copyright &copy; 2017 Bart Massey

This Rust crate implements "lazy vectors". A lazy vector stores
values at a sparse collection of indices, using storage
linear in the number of stored values. Values are
created and initialized on first assignment.

Full Rustdoc documentation is provided. GitHub makes it too
painful to actually point to it from here.

This crate requires unstable features and is only known to
work with Rust 1.23.0-nightly as of 2017-11-11. Later
versions should also work.

This is a work in progress:

* Minimal testing has been done.
* No benchmarking has been done. Slowdowns over standard
  `Vec` are likely quite substantial.
* The API will be revised to include optional
  auto-initialize on first read of an element.

This program is licensed under the "MIT License".  Please
see the file LICENSE in the source distribution of this
software for license terms.
