// original: 0x00d294b0 target_slot_first_live (proposed)

/// Run the per-slot notifier on the first live slot below the limit.
///
/// Scans the 8 slots (`this + 0x34 + i * 0x40`): the first slot whose flag
/// word is nonzero and whose float at `+0x40 + i * 0x40` is strictly below
/// 8.0 (NaN never qualifies) gets index `i` passed to the notify helper
/// (intercepted), and the function returns 1. With no such slot it returns 0.
/// Only the low byte of the result is defined.
///
/// Original: 0x00D294B0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d294b0(this: u32) -> u32 {
    unsafe {
        const FLAG_BASE: u32 = 0x34;
        const VALUE_BASE: u32 = 0x40;
        const STRIDE: u32 = 0x40;
        const COUNT: u32 = 8;
        const LIMIT_BITS: u32 = 0x4100_0000; // 8.0f
        let limit = f32::from_bits(LIMIT_BITS);
        let mut idx = 0u32;
        while idx < COUNT {
            let flag = unsafe { ((this + FLAG_BASE + idx * STRIDE) as *const u32).read_unaligned() };
            if flag != 0 {
                let v = unsafe {
                    f32::from_bits(((this + VALUE_BASE + idx * STRIDE) as *const u32).read_unaligned())
                };
                if limit > v {
                    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, idx);
                    return 1;
                }
            }
            idx += 1;
        }
        0
    }
});
