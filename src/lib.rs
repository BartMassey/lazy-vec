#![feature(unique, alloc, core_intrinsics)]

///! A "lazy array" is a self-initializing array: it can be
///! created in constant time, but still has constant-time
///! read and write. It initializes an element on first
///! write, and occupies space proportional to the number of
///! written elements.

extern crate alloc;

use std::ptr;
use alloc::raw_vec::RawVec;

///! This opaque structure stores a lazy array.
pub struct LazyArray<T> {
    // Highest index currently stored.
    size: usize,
    // Stack of actual array values.
    values: Vec<T>,
    // Parallel stack indicating, for each value, what index
    // it is located at. Used during reads and writes to
    // check for need to initialize.
    value_indices: Vec<usize>,
    // When a read or write is performed, this array is
    // indirected through to do the initialization and/or
    // access.
    indices: RawVec<usize>
}

impl <T: Copy> LazyArray<T> {

    ///! Allocate a new empty `LazyArray`.
    pub fn new() -> LazyArray<T> {
        LazyArray {
            size: 0,
            values: Vec::new(),
            value_indices: Vec::new(),
            indices: RawVec::new()
        }
    }

    ///! Allocate a new empty `LazyArray` with the given
    ///! starting index capacity.
    pub fn with_capacity(cap: usize) -> LazyArray<T> {
        LazyArray {
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

    ///! Read the value at index `i`.
    ///!
    ///! XXX For now, panic on attempt to read from an
    ///! uninitialized element.
    pub fn read(&self, i: usize) -> T {
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
        self.values[ix]
    }

    ///! Write the value `v` at index `i`.
    pub fn write(&mut self, i: usize, v: T) {
        // Get the current index capacity.
        let cap = self.indices.cap();
        // If the current index capacity is too small, grow it.
        if i >= cap {
            self.indices.reserve(cap, i - cap);
        }
        // Get a pointer to the index element.
        let ixptr = unsafe{ self.indices.ptr().offset(i as isize) };
        // Get the current index.
        let ix = unsafe{ ptr::read(ixptr) };
        // Get the stack top.
        let nstacked = self.values.len();
        assert!(nstacked == self.value_indices.len());
        // If the value is uninitialized, initialize it.
        // Otherwise, just store it.
        if ix >= nstacked || self.value_indices[ix] != i {
            // Save the value on the stack.
            self.values.push(v);
            // Save the index of the value on the stack.
            self.value_indices.push(i);
            // Save the index of the value to the index.
            unsafe{ ptr::write(ixptr, nstacked) };
            // Increase the size if necessary.
            if ix >= self.size {
                self.size = i + 1
            }
        } else {
            self.values[ix] = v
        }
    }

    #[cfg(test)]
    pub fn get_index(&self, i: usize) -> usize {
        unsafe{ ptr::read(self.indices.ptr().offset(i as isize)) }
    }

    #[cfg(test)]
    pub fn get_value_index(&self, ix: usize) -> usize {
        self.value_indices[ix]
    }
}

#[test]
fn test_basic_ops() {
    let mut a: LazyArray<i8> = LazyArray::new();
    a.write(77, -12i8);
    assert_eq!(a.read(77), -12i8);
}

#[test]
#[should_panic]
fn test_miss_off_end() {
    let mut a: LazyArray<i8> = LazyArray::new();
    a.write(77, -12i8);
    let _ = a.read(78);
}

#[test]
#[should_panic]
fn test_miss_uninit() {
    let mut a: LazyArray<i8> = LazyArray::new();
    a.write(77, -12i8);
    let _ = a.read(76);
}
