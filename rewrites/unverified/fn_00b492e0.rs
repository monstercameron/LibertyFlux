// original: 0x00B492E0 CEventShoutTargetPosition::vf8

/// Whether the shout-target-position event affects a ped: after the usual seat and task checks it always affects, first asking the compatibility helper and, when that declines, registering the shout target.
///
/// `this` points to the event, `ped` to the candidate ped. Outgoing helper
/// calls are intercepted by the contract; all comparisons are unsigned.
/// Returns 1 when the event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b492e0(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        if read_u8(ped.wrapping_add(0x210)) != 0 {
            let seat = read_u32(ped.wrapping_add(0x0A74));
            if seat == 1 || seat == 2 {
                return 0;
            }
        }
        let tasks = read_u32(ped.wrapping_add(0x224));
        if tasks == 0 {
            return 0;
        }
        let target = read_u32(this.wrapping_add(0x1C));
        if target == 0 {
            return 1;
        }
        let compatible = lf_checker_rt::callee_thiscall!(1, u32, tasks, target);
        if compatible & 0xFF != 0 {
            return 1;
        }
        lf_checker_rt::callee_thiscall!(2, u32, tasks, target, 1);
        1
    }
});
