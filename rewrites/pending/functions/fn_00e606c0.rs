// original: 0x00e606c0 timing_entry_pool_init
/// Initialise a pool of sixteen timer entries, then register the group callback.
///
/// Each entry is twelve words at `0x019F33A0 + i * 0x30`. Every entry is
/// zeroed, word 6 set to `0xFFFF_FFFF` (empty marker), word 4 set to the
/// handler address `0x006E0440`, then the entry routine (stubbed,
/// thiscall/0 with the entry address in ECX) runs and its answer is stored
/// into word 3. Finally the code pointer `0x00E6F9B0` is passed to the
/// registrar (stubbed, cdecl/1), whose answer is returned.
export!(cdecl, rw_00e606c0() -> u32 {
    unsafe {
        const ENTRIES: usize = 16;
        const WORDS: usize = 12;
        const EMPTY: u32 = 0xFFFF_FFFF;
        let base = relocated(0x19F33A0) as *mut u32;
        let handler = relocated(0x6E0440);
        for i in 0..ENTRIES {
            let e = base.add(i * WORDS);
            for w in 0..WORDS {
                e.add(w).write(0);
            }
            e.add(6).write(EMPTY);
            e.add(4).write(handler);
            let r: u32 = callee_thiscall!(1, u32, e as u32);
            e.add(3).write(r);
        }
        callee_cdecl!(2, u32, relocated(0xE6F9B0))
    }
});
