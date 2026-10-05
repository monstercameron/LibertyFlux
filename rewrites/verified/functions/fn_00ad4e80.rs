// original: 0x00AD4E80 audio_zero_pair_tables (proposed)

/// Zero the two 10000-entry audio tables at their global bases.
///
/// Writes 0x2710 zero words to each of the two tables (cdecl/0, no
/// arguments). Returns 0 (eax is zeroed before the fills and never changed).
lf_checker_rt::export!(cdecl, rw_00ad4e80() -> u32 {
    unsafe {
        const TABLE_A: u32 = 0x01552C78;
        const TABLE_B: u32 = 0x0155C8B8;
        const WORDS: usize = 0x2710;
        for i in 0..WORDS {
            lf_checker_rt::global::<u32>(TABLE_A).add(i).write(0);
            lf_checker_rt::global::<u32>(TABLE_B).add(i).write(0);
        }
        0
    }
});
