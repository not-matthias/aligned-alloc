//! Aligned memory allocator.
//!
//! Based on this: https://stackoverflow.com/a/69544158

#![feature(allocator_api)]
#![feature(new_uninit)]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_box() {
        let memory: Box<[u8], _> =
            unsafe { Box::new_zeroed_slice_in(100, AlignedAlloc::<4096>).assume_init() };

        assert_eq!(memory.as_ptr() as usize % 4096, 0);
    }
}
