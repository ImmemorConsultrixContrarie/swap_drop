#![feature(test)]
extern crate test;

use core::ptr;

/// Copy of [`swap_remove`] code.
///
/// [`swap_remove`]: std::vec::Vec::swap_remove
pub fn swap_remove<T>(self_: &mut Vec<T>, index: usize) -> T {
    #[cold]
    #[inline(never)]
    fn assert_failed(index: usize, len: usize) -> ! {
        panic!(
            "swap_remove index (is {}) should be < len (is {})",
            index, len
        );
    }

    let len = self_.len();
    if index >= len {
        assert_failed(index, len);
    }
    unsafe {
        // We replace self_[index] with the last element. Note that if the
        // bounds check above succeeds there must be a last element (which
        // can be self_[index] itself_).
        let last = ptr::read(self_.as_ptr().add(len - 1));
        let hole: *mut T = self_.as_mut_ptr().add(index);
        self_.set_len(len - 1);
        ptr::replace(hole, last)
    }
}

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
            // Will put the vector into the right
            // whether `hole` drop panics or not.
            let _guard = PanicGuard { v, index, hole };
            ptr::drop_in_place(hole);
        }
    }
}

// Tiny LCG RNG.
fn rng_next(s: u64) -> u64 {
    6364136223846793005_u64.wrapping_mul(s) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    macro_rules! default_bench {
        ($b:expr, $int:ty $(, $fn_name: expr)?) => {
            let mut rng = 43;
            let v: Vec<$int> = (0..1024).collect();
            $b.iter(|| {
                let mut v = v.clone();
                for _ in 0..512 {
                    rng = rng_next(rng);
                    let idx = (rng >> 32) % 512;
                    test::black_box($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    #[bench]
    fn bench_1000_overhead(b: &mut Bencher) {
        default_bench!(b, usize);
    }

    #[bench]
    fn bench_vec_1000_swap_remove(b: &mut Bencher) {
        default_bench!(b, usize, swap_remove);
    }

    #[bench]
    fn bench_vec_1000_swap_drop(b: &mut Bencher) {
        default_bench!(b, usize, swap_drop);
    }

    #[bench]
    fn u128_bench_1000_overhead(b: &mut Bencher) {
        default_bench!(b, u128);
    }

    #[bench]
    fn u128_bench_vec_1000_swap_remove(b: &mut Bencher) {
        default_bench!(b, u128, swap_remove);
    }

    #[bench]
    fn u128_bench_vec_1000_swap_drop(b: &mut Bencher) {
        default_bench!(b, u128, swap_drop);
    }

    macro_rules! fatstruct {
        ($b:expr $(, $fn_name:expr)?) => {
            const LEN: usize = 1024 / core::mem::size_of::<usize>();

            #[derive(Clone)]
            struct Kilobyte([usize; LEN]);
            let new = |int| Kilobyte([int; LEN]);

            let mut rng = 43;
            let v: Vec<_> = (0..1024).map(new).collect();
            $b.iter(|| {
                let mut v = v.clone();
                for _ in 0..512 {
                    rng = rng_next(rng);
                    let idx = (rng >> 32) % 512;
                    test::black_box($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    #[bench]
    fn fatstruct_bench_1000_overhead(b: &mut Bencher) {
        fatstruct!(b);
    }

    #[bench]
    fn fatstruct_bench_vec_1000_swap_remove(b: &mut Bencher) {
        fatstruct!(b, swap_remove);
    }

    #[bench]
    fn fatstruct_bench_vec_1000_swap_drop(b: &mut Bencher) {
        fatstruct!(b, swap_drop);
    }

    macro_rules! first_last {
         ($b:expr, $int:ty $(, $fn_name: expr)?) => {
            let v: Vec<$int> = (0..1024).collect();
            $b.iter(|| {
                let mut v = v.clone();
                let mut last = v.len() - 2;
                for _ in 0..512 {
                    test::black_box($($fn_name)?(&mut v, 0 as usize));
                    test::black_box($($fn_name)?(&mut v, last as usize));
                    last -= 2;
                }
                v
            })
        };
     }

    #[bench]
    fn first_last_bench_1000_overhead(b: &mut Bencher) {
        first_last!(b, usize);
    }

    #[bench]
    fn first_last_bench_vec_1000_swap_remove(b: &mut Bencher) {
        first_last!(b, usize, swap_remove);
    }

    #[bench]
    fn first_last_bench_vec_1000_swap_drop(b: &mut Bencher) {
        first_last!(b, usize, swap_drop);
    }

    #[bench]
    fn u128_first_last_bench_1000_overhead(b: &mut Bencher) {
        first_last!(b, u128);
    }

    #[bench]
    fn u128_first_last_bench_vec_1000_swap_remove(b: &mut Bencher) {
        first_last!(b, u128, swap_remove);
    }

    #[bench]
    fn u128_first_last_bench_vec_1000_swap_drop(b: &mut Bencher) {
        first_last!(b, u128, swap_drop);
    }
}
