// original: 0x00B48B40 CEventPedOnCarRoof::vf8

/// Whether the ped-on-car-roof event affects a ped: the event must name a live watcher and vehicle, the ped must sit in that vehicle, be on foot or in a matching seat state, have an active task, and the watcher must report clear.
///
/// `this` points to the event, `ped` to the candidate ped. Outgoing helper
/// calls are intercepted by the contract; all comparisons are unsigned.
/// Returns 1 when the event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b48b40(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    unsafe {
        let watcher = read_u32(this.wrapping_add(0x0C));
        if watcher == 0 {
            return 0;
        }
        let vehicle = read_u32(this.wrapping_add(0x10));
        if vehicle == 0 {
            return 0;
        }
        if read_u32(ped.wrapping_add(0x0B30)) != vehicle {
            return 0;
        }
        if read_u8(ped.wrapping_add(0x210)) != 0 {
            let seat = read_u32(ped.wrapping_add(0x0A74));
            if seat == 1 || seat == 2 {
                return 0;
            }
        }
        if read_u32(ped.wrapping_add(0x224)) == 0 {
            return 0;
        }
        let blocked = lf_checker_rt::callee_thiscall!(1, u32, watcher);
        (blocked & 0xFF == 0) as u32
    }
});
