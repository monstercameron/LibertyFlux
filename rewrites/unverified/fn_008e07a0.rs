// original: 0x008e07a0 forward_slot_pair (proposed)

/// Forward a slot's head word and its successor's head word to the setter.
///
/// Resolves slot `index` through the pool context at `CTX` (entry base at
/// `+0x00`, flag bytes at `+0x04`, stride at `+0x0c`); a set `0x80` flag bit
/// resolves to null, and the following read through it faults on both sides.
/// Otherwise reads the successor index at entry `+0x0c`: -1 means no
/// successor and the function returns without calling, while any other
/// value resolves a second entry the same way (again faulting through null
/// when its flag bit is set) and both head words go to the setter (callee
/// 1). Cdecl, one stack argument.
lf_checker_rt::export!(cdecl, rw_008e07a0(index: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const FLAG_BASE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const NEXT_OFF: u32 = 0x0c;
        const DEAD_FLAG: u8 = 0x80;
        const NO_NEXT: u32 = 0xffff_ffff;
        const CALLEE_SETTER: u32 = 1;
        let ctx = lf_checker_rt::global::<u32>(CTX).read_unaligned();
        let flag_base = ((ctx + FLAG_BASE_OFF) as *const u32).read_unaligned();
        let stride = ((ctx + STRIDE_OFF) as *const u32).read_unaligned();
        let base = (ctx as *const u32).read_unaligned();
        let entry = if (flag_base.wrapping_add(index) as *const u8).read() & DEAD_FLAG != 0 {
            0
        } else {
            base.wrapping_add(stride.wrapping_mul(index))
        };
        let next = ((entry + NEXT_OFF) as *const u32).read_unaligned();
        if next == NO_NEXT {
            return 0;
        }
        let entry2 = if (flag_base.wrapping_add(next) as *const u8).read() & DEAD_FLAG != 0 {
            (0 as *const u32).read_unaligned()
        } else {
            ((base.wrapping_add(stride.wrapping_mul(next))) as *const u32).read_unaligned()
        };
        lf_checker_rt::callee_cdecl!(
            CALLEE_SETTER,
            u32,
            (entry as *const u32).read_unaligned(),
            entry2
        );
        0
    }
});
