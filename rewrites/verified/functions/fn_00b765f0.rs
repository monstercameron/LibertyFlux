// original: 0x00b765f0 anim_state_ptr_or_lookup (proposed)

/// Return the animation-state block, looking it up when not initialised.
///
/// When the initialised flag at `this+0xbcc` is set, returns `this+0xab8`
/// directly. Otherwise reads the owner pointer at `this+0x964`: null means
/// no state (returns 0), else calls the lookup callee (thiscall on the
/// constant table 0x115dc18, one stack word: the 16-bit id at owner+0xdc)
/// and returns its answer.
///
/// Original: 0x00b765f0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b765f0(this: u32) -> u32 {
    unsafe {
        const READY_OFF: u32 = 0xbcc;
        const BLOCK_OFF: u32 = 0xab8;
        const OWNER_OFF: u32 = 0x964;
        const ID_OFF: u32 = 0xdc;
        const LOOKUP_TABLE_FILE_VA: u32 = 0x115dc18;
        const LOOKUP_CALLEE: u32 = 1;
        if ((this + READY_OFF) as *const u8).read() != 0 {
            return this + BLOCK_OFF;
        }
        let owner = ((this + OWNER_OFF) as *const u32).read_unaligned();
        if owner == 0 {
            return 0;
        }
        let id = ((owner + ID_OFF) as *const u16).read_unaligned() as u32;
        let table = lf_checker_rt::relocated(LOOKUP_TABLE_FILE_VA);
        lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, table, id)
    }
});
