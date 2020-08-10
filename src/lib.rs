#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::ptr;

/// Removes an element from the vector and drops it.
///
/// The removed element is replaced by the last element of the vector.
///
/// This does not preserve ordering, but is O(1).
///
/// Unlike [`swap_remove`] this function does not panic.
/// In case of `index >= v.len()`,
/// this function simply does nothing.
///
/// [`swap_remove`]: alloc::vec::Vec::swap_remove
///
/// # Examples
///
/// ```
/// use swap_drop::swap_drop;
///
/// let mut v = vec!["foo", "bar", "baz", "qux"];
///
/// swap_drop(&mut v, 1);
/// assert_eq!(v, ["foo", "qux", "baz"]);
///
/// v.swap_remove(0);
/// assert_eq!(v, ["baz", "qux"]);
/// ```
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
