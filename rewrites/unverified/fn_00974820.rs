// original: 0x00974820 audio_occlusion_lookup_or_tail (proposed)

/// Look up the current occlusion entry through the global manager.
///
/// Asks the manager for a count (signed compare: not-positive gives 0),
/// then for the entry (null gives 0), and otherwise tail-calls the entry
/// handler with the entry pointer. The tail call is a real call here that
/// forwards the argument and returns the result.
/// Original: 0x00974820 (cdecl, no stack arguments).
lf_checker_rt::export!(cdecl, rw_00974820() -> u32 {
    unsafe {
        const MANAGER: u32 = 0x115FD54;
        const COUNT: u32 = 1;
        const ENTRY: u32 = 2;
        const HANDLE: u32 = 3;
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read_unaligned();
        let n = lf_checker_rt::callee_thiscall!(COUNT, u32, mgr) as i32;
        if n <= 0 {
            return 0;
        }
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read_unaligned();
        let e = lf_checker_rt::callee_thiscall!(ENTRY, u32, mgr);
        if e == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(HANDLE, u32, e)
    }
});
