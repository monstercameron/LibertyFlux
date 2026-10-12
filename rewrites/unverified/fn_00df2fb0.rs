// original: 0x00DF2FB0 UIMontageText::advance_valid_text_mode

/// Read the text mode from the child at `this + 0x1e4`, skip the five modes
/// that must not be selected, then choose the next control value. The getter
/// result is an unsigned word for the skip loop and a signed 32-bit value for
/// the final comparison against eight. A signed value at least eight selects
/// mode one; every lower signed value advances by one with wrapping arithmetic.
/// The selected value and a zero flag go to the child setter, and the updated
/// view is forwarded to the numeric-label formatter by a tail call.
lf_checker_rt::export!(thiscall, rw_00df2fb0(this: u32) -> u32 {
    unsafe {
        const CHILD_FIELD: u32 = 0x1e4;
        const MODE_GETTER: u32 = 1;
        const MODE_SETTER: u32 = 2;
        const LABEL_REFRESH: u32 = 3;
        const SKIPPED_MODES: [u32; 5] = [1, 2, 3, 4, u32::MAX];
        const SIGNED_LIMIT: i32 = 8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_mode(child: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(MODE_GETTER, u32, child)
        }

        let child = read_u32(this.wrapping_add(CHILD_FIELD));
        let mut mode = read_mode(child);
        loop {
            if !SKIPPED_MODES.contains(&mode) {
                break;
            }
            mode = mode.wrapping_add(1);
        }

        let next_mode = if (mode as i32) >= SIGNED_LIMIT {
            1
        } else {
            mode.wrapping_add(1)
        };
        let _ = lf_checker_rt::callee_thiscall!(MODE_SETTER, u32, child, next_mode, 0);
        lf_checker_rt::callee_thiscall!(LABEL_REFRESH, u32, this)
    }
});
