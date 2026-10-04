// original: 0x00b0abf0 all_entries_match
/// Check that every live slot of the list reports the owner's word.
///
/// Bails out with 1 when there is no list, the module is busy, the
/// owner's word names an excluded slot, or the list is not ready.
/// Otherwise walks the eleven slots: each live slot is resolved, probed
/// and measured, and the first slot whose final reading differs from the
/// owner's word fails the check with 0. Returns 1 when every live slot
/// agrees (or no slot is live).
export!(thiscall, rw_00b0abf0(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        const WORD_OFF: usize = 0x40;
        const READY_OFF: usize = 0x26c;
        const FIRST_SLOT_OFF: usize = 0x2d4;
        const SLOT_STRIDE: usize = 12;
        const SLOT_COUNT: u32 = 11;
        const MEASURE_BASE: u32 = 0x2b0;
        const BUSY_FLAG: u32 = 0x0161_5624;
        if arg == 0 {
            return 1;
        }
        if *global::<u8>(BUSY_FLAG) != 0 {
            return 1;
        }
        let word = *((this_ptr as usize + WORD_OFF) as *const u16) as i16 as i32;
        if word == *global::<i32>(0x012F_A3E0) {
            return 1;
        }
        if word == *global::<i32>(0x0161_5640) {
            return 1;
        }
        if word == *global::<i32>(0x012F_9DA4) {
            return 1;
        }
        if word == *global::<i32>(0x012F_9DB0) {
            return 1;
        }
        if *((arg as usize + READY_OFF) as *const u8) & 4 == 0 {
            return 1;
        }
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let at = (arg as usize)
                .wrapping_add(FIRST_SLOT_OFF)
                .wrapping_add((i as usize).wrapping_mul(SLOT_STRIDE));
            let entry = *(at as *const u32);
            if entry != 0 {
                let info = callee_cdecl!(1, u32, entry);
                if info != 0 && (callee_thiscall!(2, u32, info, arg) as u8) != 0 {
                    let total =
                        callee_thiscall!(3, u32, arg.wrapping_add(MEASURE_BASE), i);
                    if (total as i32) > 0 {
                        let got = callee_thiscall!(4, u32, info);
                        if (got as i32) != word {
                            return 0;
                        }
                    }
                }
            }
            i += 1;
        }
        1
    }
});
