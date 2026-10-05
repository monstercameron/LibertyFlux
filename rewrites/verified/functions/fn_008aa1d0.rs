// original: 0x008AA1D0 audio_flag_set (proposed)

/// Set the voice active flag, adjusting the shared audio counter on change.
///
/// `flag`'s low byte is the new value for `this+0x3231`. When it differs
/// from the current byte, the shared counter (`0x115f854`) is adjusted under
/// the lock/unlock pair (`0x403cb0`/`0x403cf0`, cdecl, each taking global
/// `0x115f858`): decrement for a deactivation, increment for an activation.
/// Deactivation additionally runs the voice reset (`0x8a9580`, thiscall, no
/// arguments) first. When the flag already holds the value, nothing happens
/// besides storing it again. No return value. Original is thiscall with one
/// stack word (the callee pops 4 bytes); only the low byte of the argument is read.
lf_checker_rt::export!(thiscall, rw_008AA1D0(this: u32, flag: u32) -> u32 {
    const RESET: u32 = 1;
    const LOCK: u32 = 2;
    const UNLOCK: u32 = 3;
    const ACTIVE: u32 = 0x3231;
    const HANDLE: u32 = 0x0115_f858;
    const COUNTER: u32 = 0x0115_f854;
    unsafe {
        let want = flag as u8;
        let active = (this + ACTIVE) as *mut u8;
        if want == 0 {
            if active.read() != 0 {
                lf_checker_rt::callee_thiscall!(RESET, u32, this);
                let h = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
                lf_checker_rt::callee_cdecl!(LOCK, u32, h);
                let c = lf_checker_rt::global::<u32>(COUNTER);
                (c as *mut u32).write_unaligned((c as *const u32).read_unaligned().wrapping_sub(1));
                let h2 = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
                lf_checker_rt::callee_cdecl!(UNLOCK, u32, h2);
            }
        } else if active.read() == 0 {
            let h = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(LOCK, u32, h);
            let c = lf_checker_rt::global::<u32>(COUNTER);
            (c as *mut u32).write_unaligned((c as *const u32).read_unaligned().wrapping_add(1));
            let h2 = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(UNLOCK, u32, h2);
        }
        active.write(want);
    }
    0
});
