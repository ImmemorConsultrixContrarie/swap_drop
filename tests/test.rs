#[cfg(test)]
mod tests {
    use std::{
        mem,
        sync::{Arc, Mutex},
        thread,
    };

    use swap_drop::swap_drop;

    #[test]
    fn panic_safety_test() {
        struct Dee(i32);
        impl Drop for Dee {
            fn drop(&mut self) {
                panic!();
            }
        }

        let v: Vec<Dee> = (0..4).map(Dee).collect();
        let v = Arc::new(Mutex::new(v));
        let c = Arc::clone(&v);

        let h = thread::spawn(move || swap_drop(&mut *c.lock().unwrap(), 0));
        let _panic_err = h.join();

        let unwrap = |res| if let Ok(v) = res { v } else { panic!() };
        let unwrap_err = |res| if let Err(v) = res { v } else { panic!() };

        let v = unwrap_err(unwrap(Arc::try_unwrap(v)).into_inner()).into_inner();
        assert!(v.iter().map(|Dee(x)| x).eq([3_i32, 1, 2].iter()));
        mem::forget(v);
    }
}
