// original: 0x00B48CA0 CEventPotentialBeWalkedInto::vf8

/// Whether the potential-be-walked-into event affects a ped: the ped must be on foot or in a matching seat state, have an active task, the event must name a live target whose kind differs, and the ped must not be the excluded kind.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b48ca0(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        if read_u8(ped.wrapping_add(0x210)) != 0 {
            let seat = read_u32(ped.wrapping_add(0x0A74));
            if seat == 1 || seat == 2 {
                return 0;
            }
        }
        if read_u32(ped.wrapping_add(0x224)) == 0 {
            return 0;
        }
        let target = read_u32(this.wrapping_add(0x20));
        if target == 0 {
            return 0;
        }
        if read_u32(target.wrapping_add(0x0B80)) == 1 {
            return 0;
        }
        (read_u8(ped.wrapping_add(0x0A60)) != 2) as u32
    }
});
