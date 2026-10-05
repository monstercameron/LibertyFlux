// original: 0x00b57560 update_mode_with_tail_fini
/// Mode switch: release the old mode's helper, store the new mode, hand off.
///
/// Takes the object pointer in ECX and the new mode word on the stack.
/// When the mode is unchanged the current mode is the result. When the old
/// mode reads zero with a live helper, the helper is released first and
/// that call's answer is the result. Either way the new mode and its
/// complement are stored; when the new mode reads zero with a live helper,
/// control transfers to the shared successor with a zero word instead. A
/// release followed by a transfer cannot happen: a zero new mode always
/// skips the release.
export!(thiscall, rw_00b57560(this: u32, new_mode: u32) -> u32 {
    unsafe {
        let cur = *(this.wrapping_add(0x1188) as *const u32);
        if cur == new_mode {
            return cur;
        }
        let mut ans = cur;
        if cur == 0 {
            let helper = *(this.wrapping_add(0x1194) as *const u32);
            if helper != 0 {
                ans = callee_thiscall!(2, u32, helper, this.wrapping_add(0x117C));
            }
        }
        *(this.wrapping_add(0x1188) as *mut u32) = new_mode;
        *(this.wrapping_add(0x1184) as *mut u32) = !new_mode;
        if new_mode != 0 {
            return ans;
        }
        let helper2 = *(this.wrapping_add(0x1194) as *const u32);
        if helper2 == 0 {
            return ans;
        }
        callee_thiscall!(1, u32, helper2, 0)
    }
});
