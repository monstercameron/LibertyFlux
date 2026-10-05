// original: 0x008dfea0 pool_slot_assign (proposed)

/// Store `value` into a pool slot's head word and notify on non-zero.
///
/// Resolves slot `index` through the pool context at `CTX` (entry base at
/// `+0x00`, flag bytes at `+0x04`, stride at `+0x0c`); a set `0x80` flag bit
/// resolves to null and the store through it faults on both sides.
/// Otherwise `value` is stored at the entry head, and when non-zero the
/// notifier (callee 1) runs on the index. Returns whether the head word is
/// non-zero afterwards. Cdecl, two stack arguments.
lf_checker_rt::export!(cdecl, rw_008dfea0(index: u32, value: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const FLAG_BASE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const DEAD_FLAG: u8 = 0x80;
        const CALLEE_NOTIFY: u32 = 1;
        let ctx = lf_checker_rt::global::<u32>(CTX).read_unaligned();
        let flag_base = ((ctx + FLAG_BASE_OFF) as *const u32).read_unaligned();
        let stride = ((ctx + STRIDE_OFF) as *const u32).read_unaligned();
        let base = (ctx as *const u32).read_unaligned();
        let entry = if (flag_base.wrapping_add(index) as *const u8).read() & DEAD_FLAG != 0 {
            0
        } else {
            base.wrapping_add(stride.wrapping_mul(index))
        };
        // Kept opaque: a dead slot stores through null and faults, and the
        // compiler would otherwise fold the null store away (see fn_008e07a0).
        let entry = core::hint::black_box(entry);
        (entry as *mut u32).write_unaligned(value);
        if value != 0 {
            lf_checker_rt::callee_cdecl!(CALLEE_NOTIFY, u32, index);
        }
        u32::from((entry as *const u32).read_unaligned() != 0)
    }
});
