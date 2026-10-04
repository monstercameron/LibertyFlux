// original: 0x00e707b0 cond_release_2x1c
/// Release the two guarded slots ending at 0x01B49040.
///
/// Walks two slots of 0x1C bytes from the top down; each slot whose flag
/// word is nonzero is passed with its flag to the release callee
/// (thiscall/1). Returns the last release answer, or zero when neither
/// slot fires.
export!(cdecl, rw_00e707b0() -> u32 {
    unsafe {
        const TOP: u32 = 0x01B49040;
        const COUNT: u32 = 2;
        const STRIDE: u32 = 0x1C;
        let mut ptr = relocated(TOP);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            let flag = core::ptr::read_unaligned((ptr.wrapping_sub(STRIDE)) as *const u32);
            ptr = ptr.wrapping_sub(STRIDE);
            if flag != 0 {
                last = lf_checker_rt::callee_thiscall!(1, u32, flag, ptr);
            }
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
