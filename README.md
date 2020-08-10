A function alike to [`swap_remove`], but drops the element in place instead of returning it.

In most CPU-predictable cases (benchmarks without black box) it is faster than [`swap_remove`].
But don't trust my personal benchmarks, run `cargo bench` on the code yourself!

[`swap_remove`]: https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.swap_remove
