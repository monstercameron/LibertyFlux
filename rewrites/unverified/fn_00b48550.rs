// original: 0x00B48550 CEventMustLeaveBoat::vf8

/// Whether the must-leave-boat event affects a ped: true when the ped's linked vehicle record has the boat kind and the embarked state, or else when the ped is flagged as aboard a boat in that state.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b48550(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        let craft = read_u32(ped.wrapping_add(0x0AB0));
        if craft != 0 {
            let kind = read_u32(craft.wrapping_add(0x28)) & 0x3C0;
            if kind == 0x80 && read_u32(craft.wrapping_add(0x1304)) == 2 {
                return 1;
            }
        }
        if read_u8(ped.wrapping_add(0x26C)) & 4 == 0 {
            return 0;
        }
        let boat = read_u32(ped.wrapping_add(0x0B30));
        if boat == 0 {
            return 0;
        }
        if read_u32(boat.wrapping_add(0x1304)) != 2 {
            return 0;
        }
        1
    }
});
