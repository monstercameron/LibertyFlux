// original: 0x00DF32C0 UIMontageText::advance_secondary_text_mode

/// Read the secondary mode from the child at `this + 0x1e4`, decrement past
/// the five modes that must be skipped, and choose the preceding mode. The
/// final test is signed: a positive `i32` decrements with wrapping arithmetic,
/// while zero and negative values select eight. The setter receives the
/// selected mode and a zero flag, after which the view is forwarded to the
/// numeric-label formatter by a tail call.
lf_checker_rt::export!(thiscall, rw_00df32c0(this: u32) -> u32 {
    unsafe {
        const CHILD_FIELD: u32 = 0x1e4;
        const MODE_GETTER: u32 = 1;
        const MODE_SETTER: u32 = 2;
        const LABEL_REFRESH: u32 = 3;
        const SKIPPED_MODES: [u32; 5] = [1, 3, 4, 5, 6];
        const MODE_FLOOR: i32 = 0;
        const RESET_MODE: u32 = 8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let child = read_u32(this.wrapping_add(CHILD_FIELD));
        let mut mode = lf_checker_rt::callee_thiscall!(MODE_GETTER, u32, child);
        loop {
            if !SKIPPED_MODES.contains(&mode) {
                break;
            }
            mode = mode.wrapping_sub(1);
        }

        let previous_mode = if (mode as i32) > MODE_FLOOR {
            mode.wrapping_sub(1)
        } else {
            RESET_MODE
        };
        let _ = lf_checker_rt::callee_thiscall!(MODE_SETTER, u32, child, previous_mode, 0);
        lf_checker_rt::callee_thiscall!(LABEL_REFRESH, u32, this)
    }
});
