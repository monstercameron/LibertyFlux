// original: 0x00B49400 CEventStaticCountReachedMax::vf8

/// Whether the static-count-reached-max event affects a ped: true when the ped's count is zero or differs from its limit.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49400(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        let count = read_u32(ped.wrapping_add(0x38));
        if count == 0 {
            return 1;
        }
        (count != read_u32(ped.wrapping_add(0x07B4))) as u32
    }
});
