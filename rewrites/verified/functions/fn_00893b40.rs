// original: 0x00893b40 dispatch_state_slot (proposed)

/// Pick a work slot from the object's state word, normalising counters.
///
/// `this` points at an object with a state word at `+0x78`, a countdown at
/// `+0x54`, a selector at `+0x5c`, two counter cells at `+0x70`/`+0x74`, a
/// divisor object at `+0x7c` (divisor at its `+8`), and a shared handle at
/// `+0x38`. `arg0` is a key (its low word is stored and forwarded),
/// `arg1`/`arg2` contribute one byte each.
///
/// Gates: returns 2 when the enable flag at `+0x4d` is clear, when the
/// divisor object is null, or when the probe callee (thiscall, `arg0`)
/// answers 2. Otherwise it stamps the id bytes (`+0x8c` word, `+0x8e..0x90`
/// bytes, `+0x49` set), notifies through the first cdecl callee, then
/// dispatches on the state word when the countdown is nonzero: state 1
/// returns 1 with no further stores; state 2 clears the state, decrements
/// the countdown, folds the selector to one bit and returns 0; any other
/// state stores `arg1`'s byte, sets state 1 and returns 1. The second cdecl
/// callee is notified on every path past the gates.
///
/// When the countdown was zero (the only path that reaches here with the
/// default outcome), the selected counter cell plus one becomes the slot:
/// if `arg2`'s low byte is set, the slot and both cells are reduced modulo
/// the divisor (signed). A negative slot, or one at or above the limit
/// (signed), returns 3; a key of `0xFFFF` returns 2. Otherwise the pick
/// callee (thiscall, `arg0`, slot) answers 0..3 and selects the tail: 0 or
/// 1 returns 1, 2 returns 2, 3 runs the final callee (thiscall, `arg0`,
/// slot) and returns 1, anything higher returns 2.
///
/// Original: 0x00893b40 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00893b40(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const ENABLE: u32 = 0x4d;
        const DIV_OBJ: u32 = 0x7c;
        const HANDLE: u32 = 0x38;
        const COUNT: u32 = 0x54;
        const STATE: u32 = 0x78;
        const SELECT: u32 = 0x5c;
        const CELLS: u32 = 0x70;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd8(this + ENABLE) == 0 {
            return 2;
        }
        if rd32(this + DIV_OBJ) == 0 {
            return 2;
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, arg0);
        if probe == 2 {
            return 2;
        }
        let lo1 = arg1 as u8;
        let lo2 = arg2 as u8;
        wr8(this + 0x8e, 0);
        wr8(this + 0x8f, lo1);
        wr8(this + 0x90, lo2);
        wr16(this + 0x8c, arg0 as u16);
        wr8(this + 0x49, 1);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, rd32(this + HANDLE));
        let mut out = 2u32;
        let count = rd32(this + COUNT);
        if count != 0 {
            match rd32(this + STATE).wrapping_sub(1) {
                0 => {
                    out = 1;
                }
                1 => {
                    wr32(this + SELECT, rd32(this + SELECT).wrapping_sub(1) & 1);
                    wr32(this + STATE, 0);
                    wr32(this + COUNT, count.wrapping_sub(1));
                    out = 0;
                }
                _ => {
                    wr8(this + 0x4e, lo1);
                    wr32(this + STATE, 1);
                    out = 1;
                }
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(this + HANDLE));
        if out != 2 {
            return out;
        }
        let sel = rd32(this + SELECT).wrapping_sub(1) & 1;
        let mut slot = rd32(this + CELLS + sel * 4).wrapping_add(1);
        if lo2 != 0 {
            let div = rd32(rd32(this + DIV_OBJ) + 8) as i32;
            slot = (slot as i32).wrapping_rem(div) as u32;
            wr32(this + CELLS, (rd32(this + CELLS) as i32).wrapping_rem(div) as u32);
            wr32(this + CELLS + 4, (rd32(this + CELLS + 4) as i32).wrapping_rem(div) as u32);
        }
        if (slot as i32) < 0 {
            return 3;
        }
        if (slot as i32) >= (rd32(rd32(this + DIV_OBJ) + 8) as i32) {
            return 3;
        }
        if (arg0 as u16) == 0xFFFF {
            return 2;
        }
        let pick: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, arg0, slot);
        if pick > 3 {
            return 2;
        }
        match pick {
            0 | 1 => 1,
            2 => 2,
            _ => {
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, arg0, slot);
                1
            }
        }
    }
});
