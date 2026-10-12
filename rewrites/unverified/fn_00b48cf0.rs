// original: 0x00B48CF0 CEventPotentialGetRunOver::vf8

/// Whether the potential-get-run-over event affects a ped: the ped must be on foot or in a matching seat state, have an active task, be in a low stance, not be flagged aboard, and the event's vehicle must not be in the excluded state.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b48cf0(this: u32, ped: u32) -> u32 {
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
        if read_u8(ped.wrapping_add(0x1E2)) & 0x0F >= 2 {
            return 0;
        }
        if read_u8(ped.wrapping_add(0x26C)) & 4 != 0 {
            return 0;
        }
        let vehicle = read_u32(this.wrapping_add(0x18));
        if vehicle == 0 {
            return 0;
        }
        (read_u32(vehicle.wrapping_add(0x1300)) != 2) as u32
    }
});
