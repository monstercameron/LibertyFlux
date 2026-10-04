// original: 0x00891810 aud_param_apply_flagged
/// Applies a flag through the looked-up parameter object.
///
/// Resolves the target from the byte at 4 (0xff means null) and the table
/// index byte at 0x40, then invokes the apply callee with the target and the
/// constant flag 1. Returns the callee's answer.
export!(thiscall, rw_00891810(this: *mut u8) -> u32 {
    unsafe {
        let b = *(this.add(4));
        let target = if b == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x115d968);
            let table = *global::<u32>(0x115d988);
            let idx = *(this.add(0x40)) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f14)) as *const u32);
            stride.wrapping_mul(b as u32).wrapping_add(entry)
        };
        callee_thiscall!(1, u32, target, 1)
    }
});
