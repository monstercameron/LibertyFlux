// original: 0x00a2a680 player_ped_match_handle_entry

/// Resolve the handle through the helper, read its index at `+4`, then
/// inspect the corresponding 12-byte ped table entry at `this+0x2d4`.
/// Return AL=1 when the entry equals the expected word or is zero; otherwise
/// return whether the low byte of the fallback word is zero. Inputs are
/// `expected` then `fallback`, both callee-cleaned stack words.
lf_checker_rt::export!(thiscall, rw_a2a680(this: u32, expected: u32, fallback: u32) -> u32 {
    let handle_object = unsafe { lf_checker_rt::callee_cdecl!(1, u32, expected) };
    let index = unsafe { ((handle_object + 4) as *const u32).read_unaligned() };
    let slot_offset = 0x02d4u32.wrapping_add(index.wrapping_mul(12));
    let entry = unsafe { ((this + slot_offset) as *const u32).read_unaligned() };
    if entry == expected || entry == 0 { 1 } else { u32::from((fallback as u8) == 0) }
});
