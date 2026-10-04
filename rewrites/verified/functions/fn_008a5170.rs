// original: 0x008a5170 aud_dual_switch_tick
/// Dual-slot audio switch tick: runs the per-slot prepare/commit pair for the
/// two selector bytes at +0x48/+0x49 and folds the commit answers.
///
/// Each slot with a selector other than 0xFF resolves an object through the
/// audio table (stride global times selector plus the row cell), prepares it
/// through callee 2, then commits through callee 1 (first slot) or 3 (second
/// slot). Returns 2 if any commit answered 2, else 1 if no commit answered 0,
/// else 0. The one-bit flag forwarded to the commit call is bit 5 of +0x39.
export!(thiscall, rw_008a5170(this: *mut u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const ABSENT: u8 = 0xFF;

        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let row = *this.add(0x40) as u32;
        let slot_cell = table
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS) as *const u32;

        let mut saw_two = false;
        let mut ok = 1u8;
        let mut slot = 0;
        while slot < 2 {
            let idx = *this.add(0x48 + slot);
            if idx != ABSENT {
                let obj = stride.wrapping_mul(idx as u32).wrapping_add(*slot_cell);
                if obj != 0 {
                    let prepared: u32 = callee_thiscall!(
                        2,
                        u32,
                        obj,
                        *(this.add(0x54) as *const u32),
                        0
                    );
                    let _ = prepared;
                    let flag = ((*this.add(0x39) >> 5) & 1) as u32;
                    let target = stride.wrapping_mul(idx as u32).wrapping_add(*slot_cell);
                    let answer: u32 = if slot == 0 {
                        callee_thiscall!(1, u32, target, arg0, flag, arg1)
                    } else {
                        callee_thiscall!(3, u32, target, arg0, flag, arg1)
                    };
                    if answer == 2 {
                        saw_two = true;
                    } else if answer == 0 {
                        ok = 0;
                    }
                }
            }
            slot += 1;
        }
        if saw_two {
            2
        } else {
            ok as u32
        }
    }
});
