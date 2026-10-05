// original: 0x00AF7400 veh_pools_drain_all (proposed)

/// Drain every pool in the pool directory.
///
/// Walks the directory from `FIRST` up to (not including) `END` in steps of
/// `STEP`, asking callee 1 for each directory word's live handle. A null
/// answer skips the slot. Otherwise the slot's signed word at handle + 0x52
/// is reported to callee 2 together with the notify tag, then handed to
/// callee 3 for release. Nothing is returned.
///
/// Original: 0x00AF7400 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00AF7400() -> u32 {
    unsafe {
        const FIRST: u32 = 0x15F8BE4;
        const END: u32 = 0x15FCBA4;
        const STEP: u32 = 0x110;
        const NOTIFY_TAG: u32 = 0x104B888;
        const SLOT_WORD: u32 = 0x52;
        const GET_HANDLE: u32 = 1;
        const REPORT: u32 = 2;
        const RELEASE: u32 = 3;
        let mut slot = lf_checker_rt::relocated(FIRST);
        let end = lf_checker_rt::relocated(END);
        while slot < end {
            let word = (slot as *const u32).read_unaligned();
            let handle = lf_checker_rt::callee_cdecl!(GET_HANDLE, u32, word, 0u32);
            if handle != 0 {
                let w = ((handle + SLOT_WORD) as *const i16).read_unaligned() as i32 as u32;
                let tag = (lf_checker_rt::global::<u32>(NOTIFY_TAG) as *const u32).read_unaligned();
                lf_checker_rt::callee_cdecl!(REPORT, u32, w, tag);
                lf_checker_rt::callee_cdecl!(RELEASE, u32, w);
            }
            slot = slot.wrapping_add(STEP);
        }
        0
    }
});
