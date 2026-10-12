// original: 0x00b4b390 CEventGivePedTask::vf17

/// Decide whether this event can participate for a candidate record. A null
/// candidate is forwarded to the base event predicate. For a live candidate,
/// mode values 1 and 2, a null reference at +0x224, or a nonzero flag at
/// +0x211 make it special only when its +0x210 flag is set for the mode case.
/// A special candidate is rejected when this event's byte at +0x14 is set.
/// Other field tests are unsigned equality tests; the return is a zero-extended
/// boolean. The one stack argument is caller-cleaned by thiscall's callee.
lf_checker_rt::export!(thiscall, rw_00b4b390(this_event: u32, candidate: u32) -> u32 {
    const MODE_FLAG: u32 = 0x210;
    const CANDIDATE_MODE: u32 = 0xA74;
    const CANDIDATE_REFERENCE: u32 = 0x224;
    const CANDIDATE_FLAG: u32 = 0x211;
    const SELF_VETO: u32 = 0x14;
    const BASE_PREDICATE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    if candidate == 0 {
        // SAFETY: the contract replaces this tail jump with a scripted base call.
        return lf_checker_rt::callee_thiscall!(BASE_PREDICATE, u32, this_event, 0);
    }

    let mode_is_special = unsafe { read_u8(candidate.wrapping_add(MODE_FLAG)) } != 0
        && matches!(unsafe { read_u32(candidate.wrapping_add(CANDIDATE_MODE)) }, 1 | 2);
    let reference_is_missing = unsafe { read_u32(candidate.wrapping_add(CANDIDATE_REFERENCE)) } == 0;
    let flag_is_set = unsafe { read_u8(candidate.wrapping_add(CANDIDATE_FLAG)) } != 0;
    let candidate_is_special = mode_is_special || reference_is_missing || flag_is_set;
    if candidate_is_special && unsafe { read_u8(this_event.wrapping_add(SELF_VETO)) } != 0 {
        return 0;
    }
    1u32
});
