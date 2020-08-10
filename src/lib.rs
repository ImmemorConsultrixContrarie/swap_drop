#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::ptr;

/// Like [`swap_remove`], but does not panic with index out of bounds
/// and drops the item instead of returning it.
///
/// If index is out of bounds, simply does nothing.
///
/// [`swap_remove`]: std::vec::Vec::swap_remove
pub fn swap_drop<T>(v: &mut Vec<T>, index: usize) {
    struct PanicGuard<'a, T> {
        v: &'a mut Vec<T>,
        index: usize,
        hole: *mut T,
    }
    impl<T> Drop for PanicGuard<'_, T> {
        fn drop(&mut self) {
            unsafe {
                let last = self.v.len() - 1;
                if self.index != last {
                    ptr::write(self.hole, ptr::read(self.v.get_unchecked(last)));
                }
                self.v.set_len(last);
            }
        }
    }

    let len = v.len();
    if index < len {
        unsafe {
            let hole: *mut T = v.as_mut_ptr().add(index);
            // Will put the vector into the right state
            // whether `hole` drop panics or not.
            let _guard = PanicGuard { v, index, hole };
            ptr::drop_in_place(hole);
        }
    }
}
