// original: 0x00AD3040 audio_dispatch_by_index (proposed)

/// Dispatch an audio value to one of five channel slots by index.
///
/// With an index above 4, returns the index unchanged. Otherwise pushes the
/// value and the slot's global word and calls the slot setter; the two
/// pushed words are both compared (the setter's real stack cleanup is
/// unknown, its code is encrypted, so the proof models it as taking both).
/// Cdecl/2 (index, value); returns the setter's answer on the taken paths.
lf_checker_rt::export!(cdecl, rw_00ad3040(index: u32, value: u32) -> u32 {
    unsafe {
        const SET_SLOT: u32 = 1;
        const OWNER: u32 = 0x0154E190;
        const SLOTS: u32 = 0x0154E230;
        if index > 4 {
            return index;
        }
        let owner = lf_checker_rt::global::<u32>(OWNER).read();
        let slot = lf_checker_rt::global::<u32>(SLOTS).add(index as usize).read();
        lf_checker_rt::callee_thiscall!(SET_SLOT, u32, owner, slot, value)
    }
});
