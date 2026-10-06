// original: 0x00697a40 pool_guarded_build_passthrough (proposed)

/// Build through the pool helper under the pool mutex, passing all words.
///
/// Same lock discipline as its sibling `0x697920` (wait on the mutex word at
/// `this+0x10` when non-zero, release after), except every caller word is
/// passed to the helper unchanged: (`a1`, `a2`, `a3`, `a4`). Returns the
/// helper result's low byte (upper bytes are whatever the last call left).
///
/// Original: thiscall, four stack words, callee cleans 16.
lf_checker_rt::export!(thiscall, rw_00697a40(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const MUTEX_WORD: u32 = 0x10;
        const HELPER: u32 = 2;
        const WAIT: u32 = 10;
        const RELEASE: u32 = 11;
        const INFINITE: u32 = 0xffff_ffff;
        let mutex = (this as *const u32).byte_offset(MUTEX_WORD as isize).read_unaligned();
        if mutex != 0 {
            let _ = lf_checker_rt::callee_stdcall!(WAIT, u32, mutex, INFINITE);
        }
        let helper_ret = lf_checker_rt::callee_thiscall!(HELPER, u32, this, a1, a2, a3, a4);
        let lo = helper_ret & 0xff;
        let upper = if mutex != 0 {
            lf_checker_rt::callee_stdcall!(RELEASE, u32, mutex) & 0xffff_ff00
        } else {
            helper_ret & 0xffff_ff00
        };
        upper | lo
    }
});
