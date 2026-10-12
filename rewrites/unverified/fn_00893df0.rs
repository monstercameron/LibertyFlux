// original: 0x00893DF0 audio_handler_lookup_by_id (proposed)

/// Handler address for a one-byte effect id, or null when out of range.
///
/// Only the low byte of `id` is read. Ids 1..=24 index a jump table whose
/// entries return fixed handler addresses: id 2 returns the second handler,
/// id 8 the third, every other valid id the default handler. Id 0 and ids
/// above 24 return null. Returned addresses are relocated image addresses.
///
/// Original: 0x00893DF0 (cdecl, one stack word; low byte only).
lf_checker_rt::export!(cdecl, rw_00893df0(id: u32) -> u32 {
    const DEFAULT_HANDLER: u32 = 0x008a_3b50;
    const SECOND_HANDLER: u32 = 0x0089_b600;
    const THIRD_HANDLER: u32 = 0x0089_fda0;

    let idx = (id & 0xff).wrapping_sub(1);
    if idx > 23 {
        return 0;
    }
    let va = match idx {
        1 => SECOND_HANDLER,
        7 => THIRD_HANDLER,
        _ => DEFAULT_HANDLER,
    };
    lf_checker_rt::relocated(va)
});
