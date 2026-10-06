// original: 0x00947B80 fan_out_to_live_slots (proposed)

/// Notify every live slot, then run the finaliser, all with `cb`.
///
/// Reads the slot count byte `COUNT`. For each index below it (re-reading
/// the count every iteration), resolves the slot through callee 1 (one
/// stack word): a null answer skips the index, otherwise callee 2 runs as
/// a thiscall with the slot in ECX and `cb` on the stack. Afterwards
/// callee 3 runs with `cb` and its answer is returned. The count compares
/// are unsigned (`jb`).
///
/// Original: 0x00947B80 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00947B80(cb: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x11D74F1;
        const RESOLVE: u32 = 1;
        const NOTIFY: u32 = 2;
        const FINISH: u32 = 3;
        let n = (lf_checker_rt::relocated(COUNT) as *const u8).read();
        if n != 0 {
            let mut i = 0u32;
            loop {
                let slot = lf_checker_rt::callee_cdecl!(RESOLVE, u32, i);
                if slot != 0 {
                    lf_checker_rt::callee_thiscall!(NOTIFY, u32, slot, cb);
                }
                let m = (lf_checker_rt::relocated(COUNT) as *const u8).read();
                i += 1;
                if i >= m as u32 {
                    break;
                }
            }
        }
        lf_checker_rt::callee_cdecl!(FINISH, u32, cb)
    }
});
