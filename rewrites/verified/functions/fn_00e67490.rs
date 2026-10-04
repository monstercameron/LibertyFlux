// original: 0x00E67490 init_pool_32_entries

/// Initialise a pool of 32 fixed records from three global floats.
///
/// Loads the floats `F0..F2` once, then for each `k` in 0..32 with record base
/// `r = BASE + k * STRIDE` writes: zero at `r - 0x18`; the triple
/// `(F0, F1, F2)` at `r - 8`, `r + 8` and `r + 0x18` (each as three consecutive
/// words); zeros at `r + 0x28`, `r + 0x2C` and `r + 0x30`; `0xFFFF` at
/// `r + 0x34`; a zero byte at `r + 0x38` and a zero half-word at `r + 0x3A`.
/// Words at `r + 4`, `r + 0x14` and `r + 0x24` are left untouched. Floats move
/// as raw bits (the original's `movss` never canonicalises).
///
/// Original: 0x00E67490 (cdecl, no arguments, no outgoing calls, no return value).
lf_checker_rt::export!(cdecl, rw_00e67490() -> u32 {
    unsafe {
        const F0: u32 = 0x01B4B320;
        const F1: u32 = 0x01B4B324;
        const F2: u32 = 0x01B4B328;
        const BASE: u32 = 0x012F4558;
        const COUNT: u32 = 32;
        const STRIDE: u32 = 0x60;
        let f0 = (lf_checker_rt::global::<u32>(F0)).read_unaligned();
        let f1 = (lf_checker_rt::global::<u32>(F1)).read_unaligned();
        let f2 = (lf_checker_rt::global::<u32>(F2)).read_unaligned();
        let mut r = lf_checker_rt::relocated(BASE);
        let mut k: u32 = 0;
        while k < COUNT {
            ((r - 0x18) as *mut u32).write_unaligned(0);
            ((r - 8) as *mut u32).write_unaligned(f0);
            ((r - 4) as *mut u32).write_unaligned(f1);
            (r as *mut u32).write_unaligned(f2);
            ((r + 8) as *mut u32).write_unaligned(f0);
            ((r + 0x0c) as *mut u32).write_unaligned(f1);
            ((r + 0x10) as *mut u32).write_unaligned(f2);
            ((r + 0x18) as *mut u32).write_unaligned(f0);
            ((r + 0x1c) as *mut u32).write_unaligned(f1);
            ((r + 0x20) as *mut u32).write_unaligned(f2);
            ((r + 0x28) as *mut u32).write_unaligned(0);
            ((r + 0x2c) as *mut u32).write_unaligned(0);
            ((r + 0x30) as *mut u32).write_unaligned(0);
            ((r + 0x34) as *mut u32).write_unaligned(0xffff);
            ((r + 0x38) as *mut u8).write_unaligned(0);
            ((r + 0x3a) as *mut u16).write_unaligned(0);
            r = r.wrapping_add(STRIDE);
            k += 1;
        }
    }
    0
});
