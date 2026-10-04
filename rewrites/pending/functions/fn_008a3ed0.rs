// original: 0x008a3ed0 aud_switch_update
/// Audio switch update: refresh the active voice or probe the idle one.
///
/// When +0xF1 is set, the voice key from the scaled word at +0x3C (callee 1)
/// and the resolved object are committed through callee 2, which also gets
/// the object pointer patched in its low byte with bit 5 of +0x39. A commit
/// answer of 2 releases through callee 3, clears +0xF1 and returns 0; an
/// answer of 1 chains callees 4/5/4/6/7, clears +0xF1 and returns 1; any
/// other answer returns 1 with nothing further. When +0xF1 is clear, the
/// idle voice is probed through callee 8 (skipped when the selector is 0xFF
/// or the resolution is null): 0 is returned when +0xF2 is clear and the
/// probe was skipped or answered zero, otherwise 1, calling callee 7 first
/// when +0xF2 is clear but the probe answered nonzero. Only AL is defined.
export!(thiscall, rw_008a3ed0(this: *mut u8, arg0: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const ABSENT: u8 = 0xFF;

        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let row = *this.add(0x40) as u32;
        let cell = *((table
            .wrapping_add(row.wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS)) as *const u32);

        if *this.add(0xF1) != 0 {
            let flag = ((*this.add(0x39) >> 5) & 1) as u32;
            let scaled = *(this.add(0x3c) as *const i16) as i32 as u32;
            let key: u32 = callee_cdecl!(1, u32, scaled);
            let idx = *this.add(0x48);
            let target = if idx == ABSENT {
                0
            } else {
                stride.wrapping_mul(idx as u32).wrapping_add(cell)
            };
            let patched = (this as u32 & 0xFFFFFF00) | flag;
            let answer: u32 = callee_thiscall!(2, u32, target, key, patched, 0);
            if answer == 2 {
                let _: u32 = callee_thiscall!(3, u32, this as u32, 0);
                *this.add(0xF1) = 0;
                return 0;
            }
            if answer != 1 {
                return 1;
            }
            *this.add(0xF1) = 0;
            let a1: u32 = callee_thiscall!(4, u32, this as u32, 0);
            let _: u32 = callee_thiscall!(
                5,
                u32,
                a1,
                *(this.add(0x54) as *const u32),
                0
            );
            let a2: u32 = callee_thiscall!(4, u32, this as u32, 0);
            let _: u32 = callee_thiscall!(6, u32, a2, arg0);
            let _: u32 = callee_thiscall!(7, u32, this as u32);
            return 1;
        }

        let idx = *this.add(0x48);
        let mut skipped_or_zero = true;
        if idx != ABSENT {
            let v = stride.wrapping_mul(idx as u32).wrapping_add(cell);
            if v != 0 {
                let ans: u32 = callee_thiscall!(8, u32, v, arg0);
                skipped_or_zero = (ans as u8) == 0;
            }
        }
        if skipped_or_zero && *this.add(0xF2) == 0 {
            return 0;
        }
        if *this.add(0xF2) != 0 {
            return 1;
        }
        let _: u32 = callee_thiscall!(7, u32, this as u32);
        1
    }
});
