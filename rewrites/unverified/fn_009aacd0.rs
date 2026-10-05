// original: 0x009AACD0 audio_table_flag_set (proposed)

/// Audio table-flag set: looks `p` up in a global id table, then marks two
/// state flags on the voice object `this`.
///
/// When `p` is null nothing happens. Otherwise the id-table callee is asked
/// about (`ID_TABLE`, 10, `p`) and bytes at `this + 0x4f8` and `this + 0x504`
/// are set to 3 (`FLAG_DONE`). The callee's answer is ignored. Returns nothing.
/// Original: 0x009AACD0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009AACD0(this: u32, p: u32) -> u32 {
    unsafe {
        const ID_TABLE: u32 = 0x012885a8;
        const TABLE_N: u32 = 10;
        const FLAG_A: u32 = 0x4f8;
        const FLAG_B: u32 = 0x504;
        const FLAG_DONE: u8 = 3;
        const LOOKUP: u32 = 1;
        if p != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                LOOKUP,
                u32,
                lf_checker_rt::relocated(ID_TABLE),
                TABLE_N,
                p
            );
            ((this.wrapping_add(FLAG_A)) as *mut u8).write(FLAG_DONE);
            ((this.wrapping_add(FLAG_B)) as *mut u8).write(FLAG_DONE);
        }
        0
    }
});
