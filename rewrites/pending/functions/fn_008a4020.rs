// original: 0x008a4020 rage::audSwitchSound::vf9
/// rage::audSwitchSound::vf9: resolve the indexed switch voice and commit it.
///
/// Reads the selector byte at this+arg1+0x48 (arg1 is a small field offset).
/// Absent (0xFF) clears +0xF2 and returns entry EAX, which the contract
/// defines as the object pointer since the original leaves EAX untouched on
/// that path. A null resolution clears +0xF2 and returns 0. Otherwise the
/// voice is prepared (callee 2), keyed by the scaled word at +0x3C (callee 3),
/// and committed (callee 1) with the key, arg1 patched in its low byte with
/// bit 5 of +0x39, and zero. A commit answer other than 1 stores the word 1
/// at +0xF1 and is returned; an answer of 1 falls through to callee 4 whose
/// answer is returned.
export!(thiscall, rw_008a4020(this: *mut u8, arg0: u32, arg1: u32) -> u32 {
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
        let at = (arg1 as usize).wrapping_add(0x48);

        let idx = *this.add(at);
        if idx == ABSENT {
            *this.add(0xF2) = 0;
            return this as u32;
        }
        let obj = stride.wrapping_mul(idx as u32).wrapping_add(cell);
        if obj == 0 {
            *this.add(0xF2) = 0;
            return 0;
        }
        let prepared: u32 = callee_thiscall!(
            2,
            u32,
            obj,
            *(this.add(0x54) as *const u32),
            0
        );
        let _ = prepared;
        let flag = ((*this.add(0x39) >> 5) & 1) as u32;
        let scaled = *(this.add(0x3c) as *const i16) as i32 as u32;
        let key: u32 = callee_cdecl!(3, u32, scaled);
        let idx2 = *this.add(at);
        let target = if idx2 == ABSENT {
            0
        } else {
            stride.wrapping_mul(idx2 as u32).wrapping_add(cell)
        };
        let arg1p = (arg1 & 0xFFFFFF00) | flag;
        let answer: u32 = callee_thiscall!(1, u32, target, key, arg1p, 0);
        if answer != 1 {
            *(this.add(0xF1) as *mut u16) = 1;
            return answer;
        }
        let idx3 = *this.add(at);
        let target3 = if idx3 == ABSENT {
            0
        } else {
            stride.wrapping_mul(idx3 as u32).wrapping_add(cell)
        };
        let out: u32 = callee_thiscall!(4, u32, target3, arg0);
        *this.add(0xF2) = 0;
        out
    }
});
