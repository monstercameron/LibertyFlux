// original: 0x00892010 aud_sound_update
/// Updates a sound's state machine and returns the outcome code.
///
/// Returns 1 at once when the active bit (bit 7 of the byte at 0x38) is set.
/// A zero first argument is replaced by the index callee's answer for the
/// word at 0x3c. When the third argument's low byte is zero, the second
/// argument's low byte (shifted left by 5 and masked to bit 5 after mixing
/// with the flag byte) is folded into the byte at 0x39, state
/// 1 is stamped, and the word at 0x3c is refreshed from the refresh callee.
/// A true chain probe returns 1. Otherwise the dispatch table entry selected
/// by the byte at 0x3b runs with this object and the first two arguments; a
/// nonzero third-argument low byte returns its answer at once, an answer of 1
/// resets the state and sets the active bit, 2 sets flag bit 0, and anything
/// else clears the active bit.
export!(thiscall, rw_00892010(this: *mut u8, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        if *(this.add(0x38)) & 0x80 != 0 {
            return 1;
        }
        let mut first = a1;
        if first == 0 {
            let w = *(this.add(0x3c) as *const i16) as i32 as u32;
            first = callee_cdecl!(1, u32, w);
        }
        let bl = a3 as u8;
        if bl == 0 {
            let mut cl = (a2 as u8).wrapping_shl(5);
            cl ^= *(this.add(0x39));
            cl &= 0x20;
            *(this.add(0x39)) ^= cl;
            *(this.add(6) as *mut u16) = 1;
            let v: u32 = callee_cdecl!(2, u32, first);
            *(this.add(0x3c) as *mut u16) = v as u16;
        }
        let probe: u32 = callee_thiscall!(3, u32, this as u32);
        if probe & 0xff != 0 {
            return 1;
        }
        let idx = *(this.add(0x3b)) as u32;
        let base = relocated(0x115d654);
        let callee = *((base.wrapping_add(idx.wrapping_mul(4))) as *const u32);
        let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee as usize);
        first = f(this as u32, first, a3);
        if bl != 0 {
            return first;
        }
        if first == 1 {
            *(this.add(6) as *mut u16) = 0;
            let _: u32 = callee_thiscall!(4, u32, this as u32);
            *(this.add(0x38)) |= 0x80;
            return first;
        }
        if first == 2 {
            *(this.add(0x39)) |= 1;
        } else {
            *(this.add(0x38)) &= 0x7f;
        }
        first
    }
});
