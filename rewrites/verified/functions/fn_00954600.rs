// original: 0x00954600 id_range_or_removed (proposed)

/// Accept a 16-bit id, or the removed marker.
///
/// Returns 1 when the low 16 bits of `value` are at most `MAX_ID`
/// (compared as an UNSIGNED 16-bit value), otherwise returns whether the
/// whole 32-bit `value` equals `REMOVED` (0xFFFFFFFE). The high 16 bits
/// are ignored on the first path. Original is cdecl/1, returns AL.
lf_checker_rt::export!(cdecl, rw_00954600(value: u32) -> u32 {
    const MAX_ID: u16 = 0x817;
    const REMOVED: u32 = 0xFFFFFFFE;
    if (value as u16) > MAX_ID {
        (value == REMOVED) as u32
    } else {
        1
    }
});
