// original: 0x00697920 pool_guarded_build (proposed)

/// Build through the pool helper under the pool mutex, with a fixed callback.
///
/// When the mutex word at `this+0x10` is non-zero it is waited on
/// (`WaitForSingleObject(mutex, INFINITE)`) before the helper runs and
/// released after; a zero word skips both. The helper always runs with
/// (`a1`, callback `0x697750`, `a3`, `a4`): the caller's second word is
/// replaced by the callback address. Returns the helper result's low byte;
/// the upper bytes are the release call's when the mutex was taken, else
/// zero (the mutex word is reloaded into `eax` after the helper call).
///
/// Original: thiscall, four stack words, callee cleans 16.
lf_checker_rt::export!(thiscall, rw_00697920(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const MUTEX_WORD: u32 = 0x10;
        const CALLBACK_FILE_VA: u32 = 0x00697750;
        const HELPER: u32 = 2;
        const WAIT: u32 = 10;
        const RELEASE: u32 = 11;
        const INFINITE: u32 = 0xffff_ffff;
        let _ = a2;
        let mutex = (this as *const u32).byte_offset(MUTEX_WORD as isize).read_unaligned();
        if mutex != 0 {
            let _ = lf_checker_rt::callee_stdcall!(WAIT, u32, mutex, INFINITE);
        }
        // The original pushes this address as an immediate with a relocation
        // entry, so it arrives relocated; match that, not the file value.
        let callback = lf_checker_rt::relocated(CALLBACK_FILE_VA);
        let helper_ret = lf_checker_rt::callee_thiscall!(HELPER, u32, this, a1, callback, a3, a4);
        let lo = helper_ret & 0xff;
        if mutex != 0 {
            let release_ret = lf_checker_rt::callee_stdcall!(RELEASE, u32, mutex);
            (release_ret & 0xffff_ff00) | lo
        } else {
            lo
        }
    }
});
