//! Aligned memory allocator.
//!
//! Based on this: https://stackoverflow.com/a/69544158

#![feature(allocator_api)]

extern crate alloc;

use alloc::alloc::Global;
use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

pub struct AlignedAlloc<const N: usize>;

unsafe impl<const N: usize> Allocator for AlignedAlloc<N> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Global.allocate(layout.align_to(N).unwrap())
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout.align_to(N).unwrap())
    }
}
