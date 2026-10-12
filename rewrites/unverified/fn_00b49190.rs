// original: 0x00B49190 CEventScriptCommand::vf8

/// Whether the script-command event affects a ped: the ped must not be flagged busy, must be on foot or in a matching seat state, and must either have an active task or be named by the event.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49190(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        if read_u8(ped.wrapping_add(0x211)) != 0 {
            return 0;
        }
        if read_u8(ped.wrapping_add(0x210)) != 0 {
            let seat = read_u32(ped.wrapping_add(0x0A74));
            if seat == 1 || seat == 2 {
                return 0;
            }
        }
        if read_u32(ped.wrapping_add(0x224)) != 0 {
            return 1;
        }
        (read_u8(this.wrapping_add(0x14)) != 0) as u32
    }
});
