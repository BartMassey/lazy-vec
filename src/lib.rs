#![feature(unique, alloc, core_intrinsics)]

///! A "lazy vector" is a self-initializing vector: it can be
///! created in constant time, but still has constant-time
///! read and write. It initializes an element on first
///! write, and occupies space proportional to the number of
///! written elements.
///!
///! # Examples
///! ```
///! use lazy_vec::LazyVec;
///! 
///! let mut a: LazyVec<i8> = LazyVec::new();
///! a[77] = -12i8;
///! assert_eq!(a[77], -12i8);
///! ```

extern crate alloc;

use std::ptr;
use alloc::raw_vec::RawVec;
use std::ops::{Index, IndexMut};

///! This opaque structure stores a lazy vector.
pub struct LazyVec<T> {
    // Highest index currently stored.
    size: usize,
    // Stack of actual vector values.
    values: Vec<T>,
    // Parallel stack indicating, for each value, what index
    // it is located at. Used during reads and writes to
    // check for need to initialize.
    value_indices: Vec<usize>,
    // When a read or write is performed, this vector is
    // indirected through to do the initialization and/or
    // access.
    indices: RawVec<usize>
}

impl <T: Copy> LazyVec<T> {

    ///! Allocate a new empty `LazyVec`.
    pub fn new() -> LazyVec<T> {
        LazyVec {
            size: 0,
            values: Vec::new(),
            value_indices: Vec::new(),
            indices: RawVec::new()
        }
    }

    ///! Allocate a new empty `LazyVec` with the given
    ///! starting index capacity.
    pub fn with_capacity(cap: usize) -> LazyVec<T> {
        LazyVec {
            size: 0,
            values: Vec::new(),
            value_indices: Vec::new(),
            indices: RawVec::with_capacity(cap)
        }
    }

    ///! Number of elements there is currently notional
    ///! capacity for, including uninitialized ones.
    pub fn cap(&self) -> usize {
        self.indices.cap()
    }

    ///! Number of elements notionally stored, including
    ///! uninitialized ones.
    pub fn len(&self) -> usize {
        self.size
    }

    ///! Return a reference to the value at the given index.
    ///!
    ///! XXX For now, panic on attempt to read from an
    ///! uninitialized element.
    pub fn value_ref(&self, i: usize) -> &T {
        // XXX For now, fail if the value has not been
        // initialized (index off end of indices).
        assert!(i < self.size);
        // Get the putative index into the value stack.
        let ix = unsafe {
            // Get the correct pointer.
            let ixptr = self.indices.ptr().offset(i as isize);
            // Read the value there.
            ptr::read(ixptr)
        };
        // XXX For now, fail if the value has not been
        // initialized (index off end of value stack).
        assert!(ix < self.values.len());
        // XXX For now, fail if the value has not been
        // initialized (index does not point to valid
        // value).
        assert!(self.value_indices[ix] == i);
        // Return the correct value from the value stack.
        &self.values[ix]
    }

    ///! Return a mutable reference to the value at index `i`.
    ///! If no value previously existed, this will return a reference
    ///! to uninitialized memory, making it unsafe.
    pub unsafe fn value_ref_mut(&mut self, i: usize) -> &mut T {
        // Get the current index capacity.
        let cap = self.indices.cap();
        // If the current index capacity is too small, grow it.
        if i >= cap {
            self.indices.reserve(cap, i - cap);
        }
        // Get a pointer to the index element.
        let ixptr = self.indices.ptr().offset(i as isize);
        // Get the current index.
        let ix = ptr::read(ixptr);
        // Get the stack top.
        let nstacked = self.values.len();
        assert!(nstacked == self.value_indices.len());
        // If the value is uninitialized, initialize it.
        // Otherwise, just store it.
        if ix >= nstacked || self.value_indices[ix] != i {
            // Save a place for a value on the stack.
            self.values.reserve(1);
            self.values.set_len(nstacked + 1);
            // Save the index of the value on the stack.
            self.value_indices.push(i);
            // Save the index of the value to the index.
            ptr::write(ixptr, nstacked);
            // Increase the size if necessary.
            if ix >= self.size {
                self.size = i + 1
            };
            &mut self.values[nstacked];
        };
        &mut self.values[ix]
    }
}

impl<T: Copy> Index<usize> for LazyVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &T {
        self.value_ref(i)
    }
}

impl<T: Copy> IndexMut<usize> for LazyVec<T> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        unsafe{ self.value_ref_mut(i) }
    }
}

#[test]
#[should_panic]
fn test_miss_off_end() {
    let mut a: LazyVec<i8> = LazyVec::new();
    a[77] = -12i8;
    let _ = a[100000];
}

#[test]
#[should_panic]
fn test_miss_uninit() {
    let mut a: LazyVec<i8> = LazyVec::new();
    a[77] = -12i8;
    let _ = a[76];
}
