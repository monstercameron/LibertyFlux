// original: 0x00AF8E30 veh_zone_slot_ptr (proposed)

/// Address the zone record for a slot index.
///
/// Returns `ZONE_BASE + index * 40` with wrapping arithmetic, where each
/// 40-byte record starts at file address `0x15FCCE0`. The value is computed
/// as pure arithmetic: the original adds the unrelocated base, so the
/// rewrite returns the identical value rather than a relocated pointer.
///
/// Original: 0x00AF8E30 (cdecl, one stack argument, address in EAX).
lf_checker_rt::export!(cdecl, rw_00AF8E30(index: u32) -> u32 {
    const ZONE_BASE: u32 = 0x15FC_CE0;
    const RECORD_LEN: u32 = 40;
    ZONE_BASE.wrapping_add(index.wrapping_mul(RECORD_LEN))
});
