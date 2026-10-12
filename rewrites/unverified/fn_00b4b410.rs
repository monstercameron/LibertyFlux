// original: 0x00b4b410 CEventMustLeaveBoat::vf17

/// Return false for a live candidate whose +0x210 flag is set and mode is 1
/// or 2, or whose reference word at +0x224 is zero; otherwise return true.
/// A null candidate is forwarded to the base event predicate through its
/// scripted tail call. This method takes one thiscall stack argument.
lf_checker_rt::export!(thiscall, rw_00b4b410(this_event: u32, candidate: u32) -> u32 {
    const MODE_FLAG: u32 = 0x210;
    const CANDIDATE_MODE: u32 = 0xA74;
    const CANDIDATE_REFERENCE: u32 = 0x224;
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
    let mode_flag = unsafe { read_u8(candidate.wrapping_add(MODE_FLAG)) };
    let mode_is_special = if mode_flag != 0 {
        let mode = unsafe { read_u32(candidate.wrapping_add(CANDIDATE_MODE)) };
        mode == 1 || mode == 2
    } else {
        false
    };
    let reference_is_missing = unsafe { read_u32(candidate.wrapping_add(CANDIDATE_REFERENCE)) } == 0;
    u32::from(!(mode_is_special || reference_is_missing))
});
