#![feature(test)]
extern crate test;

use swap_drop::swap_drop;

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

// Tiny LCG RNG.
fn rng_next(s: u64) -> u64 {
    6364136223846793005_u64.wrapping_mul(s) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    macro_rules! default_bench {
        ($black:expr, $b:expr, $int:ty $(, $fn_name: expr)?) => {
            let mut rng = 43;
            let v: Vec<$int> = (0..1024).collect();
            $b.iter(|| {
                let mut v = v.clone();
                for _ in 0..512 {
                    rng = rng_next(rng);
                    let idx = (rng >> 32) % 512;
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    macro_rules! fatstruct {
        ($black:expr, $b:expr $(, $fn_name:expr)?) => {
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
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    macro_rules! first_last {
        ($black:expr, $b:expr, $int:ty $(, $fn_name: expr)?) => {
            let v: Vec<$int> = (0..1024).collect();
            $b.iter(|| {
                let mut v = v.clone();
                let mut last = v.len() - 2;
                for _ in 0..512 {
                    let idx = 0;
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                    let idx = last;
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                    last -= 2;
                }
                v
            })
        };
     }

    macro_rules! first_only {
        ($black:expr, $b:expr, $int:ty $(, $fn_name: expr)?) => {
            let v: Vec<$int> = (0..1024).collect();
            $b.iter(|| {
                let mut v = v.clone();
                for _ in 0..512 {
                    let idx = 0;
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    macro_rules! last_only {
        ($black:expr, $b:expr, $int:ty $(, $fn_name: expr)?) => {
            let v: Vec<$int> = (0..1024).collect();
            $b.iter(|| {
                let mut v = v.clone();
                for idx in (0..v.len()).rev() {
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    macro_rules! droppable {
        ($black:expr, $b:expr, $int:ty $(, $fn_name: expr)?) => {
            let mut rng = 43;
            let v: Vec<_> = (0..1024).map(|i| vec![i; 4]).collect();
            $b.iter(|| {
                let mut v = v.clone();
                for _ in 0..512 {
                    rng = rng_next(rng);
                    let idx = (rng >> 32) % 512;
                    let idx = if $black { test::black_box(idx) } else { idx };
                    ($($fn_name)?(&mut v, idx as usize));
                }
                v
            })
        };
    }

    #[bench]
    fn bench_swap_remove(b: &mut Bencher) {
        default_bench!(true, b, usize, swap_remove);
    }

    #[bench]
    fn bench_swap_drop(b: &mut Bencher) {
        default_bench!(true, b, usize, swap_drop);
    }

    #[bench]
    fn u128_swap_remove(b: &mut Bencher) {
        default_bench!(true, b, u128, swap_remove);
    }

    #[bench]
    fn u128_swap_drop(b: &mut Bencher) {
        default_bench!(true, b, u128, swap_drop);
    }

    #[bench]
    fn fatstruct_swap_remove(b: &mut Bencher) {
        fatstruct!(true, b, swap_remove);
    }

    #[bench]
    fn fatstruct_swap_drop(b: &mut Bencher) {
        fatstruct!(true, b, swap_drop);
    }

    #[bench]
    fn first_last_swap_remove(b: &mut Bencher) {
        first_last!(true, b, usize, swap_remove);
    }

    #[bench]
    fn first_last_swap_drop(b: &mut Bencher) {
        first_last!(true, b, usize, swap_drop);
    }

    #[bench]
    fn u128_first_last_swap_remove(b: &mut Bencher) {
        first_last!(true, b, u128, swap_remove);
    }

    #[bench]
    fn u128_first_last_swap_drop(b: &mut Bencher) {
        first_last!(true, b, u128, swap_drop);
    }

    #[bench]
    fn first_only_swap_remove(b: &mut Bencher) {
        first_only!(true, b, usize, swap_remove);
    }

    #[bench]
    fn first_only_swap_drop(b: &mut Bencher) {
        first_only!(true, b, usize, swap_drop);
    }

    #[bench]
    fn last_only_swap_remove(b: &mut Bencher) {
        last_only!(true, b, usize, swap_remove);
    }

    #[bench]
    fn last_only_swap_drop(b: &mut Bencher) {
        last_only!(true, b, usize, swap_drop);
    }

    #[bench]
    fn droppable_items_swap_remove(b: &mut Bencher) {
        droppable!(true, b, usize, swap_remove);
    }

    #[bench]
    fn droppable_items_swap_drop(b: &mut Bencher) {
        droppable!(true, b, usize, swap_drop);
    }

    #[bench]
    fn no_blackbox_swap_remove(b: &mut Bencher) {
        default_bench!(false, b, usize, swap_remove);
    }

    #[bench]
    fn no_blackbox_swap_drop(b: &mut Bencher) {
        default_bench!(false, b, usize, swap_drop);
    }

    #[bench]
    fn no_blackbox_u128_swap_remove(b: &mut Bencher) {
        default_bench!(false, b, u128, swap_remove);
    }

    #[bench]
    fn no_blackbox_u128_swap_drop(b: &mut Bencher) {
        default_bench!(false, b, u128, swap_drop);
    }

    #[bench]
    fn no_blackbox_fatstruct_swap_remove(b: &mut Bencher) {
        fatstruct!(false, b, swap_remove);
    }

    #[bench]
    fn no_blackbox_fatstruct_swap_drop(b: &mut Bencher) {
        fatstruct!(false, b, swap_drop);
    }

    #[bench]
    fn no_blackbox_first_last_swap_remove(b: &mut Bencher) {
        first_last!(false, b, usize, swap_remove);
    }

    #[bench]
    fn no_blackbox_first_last_swap_drop(b: &mut Bencher) {
        first_last!(false, b, usize, swap_drop);
    }

    #[bench]
    fn no_blackbox_u128_first_last_swap_remove(b: &mut Bencher) {
        first_last!(false, b, u128, swap_remove);
    }

    #[bench]
    fn no_blackbox_u128_first_last_swap_drop(b: &mut Bencher) {
        first_last!(false, b, u128, swap_drop);
    }

    #[bench]
    fn no_blackbox_first_only_swap_remove(b: &mut Bencher) {
        first_only!(false, b, usize, swap_remove);
    }

    #[bench]
    fn no_blackbox_first_only_swap_drop(b: &mut Bencher) {
        first_only!(false, b, usize, swap_drop);
    }

    #[bench]
    fn no_blackbox_last_only_swap_remove(b: &mut Bencher) {
        last_only!(false, b, usize, swap_remove);
    }

    #[bench]
    fn no_blackbox_last_only_swap_drop(b: &mut Bencher) {
        last_only!(false, b, usize, swap_drop);
    }

    #[bench]
    fn no_blackbox_droppable_items_swap_remove(b: &mut Bencher) {
        droppable!(false, b, usize, swap_remove);
    }

    #[bench]
    fn no_blackbox_droppable_items_swap_drop(b: &mut Bencher) {
        droppable!(false, b, usize, swap_drop);
    }
}
