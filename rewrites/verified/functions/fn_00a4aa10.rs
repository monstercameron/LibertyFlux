// original: 0x00A4AA10 vehicle_id_in_table_a (proposed)

/// True when the argument equals any of thirteen global id words.
///
/// Compares the argument against thirteen dword globals in `.data` (all
/// relocated; pristine zero) in order and returns 1 on the first match, else
/// 0. Called directly and as the tail target of the neighbouring seven-word
/// check. No writes, no calls.
///
/// Original: 0x00A4AA10 (cdecl, one stack word), leaf.
lf_checker_rt::export!(cdecl, rw_00A4AA10(val: u32) -> u32 {
    unsafe {
        const IDS: [u32; 13] = [
            0x012F9FB4, 0x012F9FF0, 0x012FA17C, 0x012FA2FC, 0x012FA068,
            0x012FA314, 0x012FA494, 0x012FA044, 0x012F9DBC, 0x012FA62C,
            0x012F9F24, 0x012FA284, 0x012FA3A4,
        ];
        for file_va in IDS {
            if lf_checker_rt::global::<u32>(file_va).read_unaligned() == val {
                return 1;
            }
        }
        0
    }
});
