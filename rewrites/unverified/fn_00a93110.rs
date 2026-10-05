// original: 0x00a93110 stream_mark_slots_equal (proposed)

/// Mark every live set slot whose tag word equals the argument.
///
/// Each slot index below the set count at `set+0x08` (set from its global)
/// whose select byte (`base[i]`, base at `set+0x04`) has bit 7 clear and
/// whose address (`set+0x00 + stride*i`, stride at `set+0x0c`) is nonzero
/// and holds the argument in its tag word at `+0x50` is marked (callee 1,
/// cdecl/2 with the index and a global flag).
///
/// Returns the last callee answer, the slot address when the last slot's
/// tag differed, the select base or 0 for the skip paths, or 0 when no slot
/// ran (entry eax is pinned to 0 by the contract). Cdecl, one argument.
lf_checker_rt::export!(cdecl, rw_00a93110(want: u32) -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const FLAG_GLOBAL: u32 = 0x0103e89c;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const COUNT_OFF: u32 = 0x08;
        const STRIDE_OFF: u32 = 0x0c;
        const TAG_OFF: u32 = 0x50;
        const SKIP_BIT: u8 = 0x80;
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        if ((set + COUNT_OFF) as *const i32).read_unaligned() <= 0 {
            return 0;
        }
        let mut ret = 0u32;
        let mut i = 0i32;
        while i < ((set + COUNT_OFF) as *const i32).read_unaligned() {
            let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
            if (((selbase + (i as u32)) as *const u8).read() & SKIP_BIT) != 0 {
                ret = selbase;
            } else {
                let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
                let base = ((set + BASE_OFF) as *const u32).read_unaligned();
                let slot = base.wrapping_add(stride.wrapping_mul(i as u32));
                if slot == 0 {
                    ret = 0;
                } else if ((slot + TAG_OFF) as *const u32).read_unaligned() != want {
                    ret = slot;
                } else {
                    let flag =
                        lf_checker_rt::global::<u32>(FLAG_GLOBAL).read_unaligned();
                    ret = lf_checker_rt::callee_cdecl!(1, u32, i as u32, flag);
                }
            }
            i += 1;
        }
        ret
    }
});
