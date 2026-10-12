// original: 0x00B49420 CEventStuckInAir::vf8

/// Whether the stuck-in-air event affects a ped: the ped must not be flagged grounded and its task record must carry a nonzero air word.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49420(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        if read_u8(ped.wrapping_add(0x26C)) & 1 != 0 {
            return 0;
        }
        let tasks = read_u32(ped.wrapping_add(0x224));
        (read_u16(tasks.wrapping_add(0x244)) != 0) as u32
    }
});
