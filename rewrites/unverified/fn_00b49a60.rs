// original: 0x00B49A60 CEventVehicleToSteal::vf8

/// Whether the vehicle-to-steal event affects a ped: after the usual seat and task checks the event must name a vehicle whose driver task is idle; when the current player's vehicle matches, two mission helpers must also decline; then the ped's state word selects the match rule.
///
/// `this` points to the event, `ped` to the candidate ped. Outgoing helper
/// calls are intercepted by the contract; all comparisons are unsigned.
/// Returns 1 when the event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49a60(this: u32, ped: u32) -> u32 {
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
        if read_u32(ped.wrapping_add(0x224)) == 0 {
            return 0;
        }
        let vehicle = read_u32(this.wrapping_add(0x0C));
        if vehicle == 0 {
            return 0;
        }
        let player = lf_checker_rt::callee_cdecl!(1, u32, 0);
        let found = lf_checker_rt::callee_thiscall!(2, u32, read_u32(player.wrapping_add(0x224)).wrapping_add(0x44), 3, 0x2DE);
        if found != 0 && read_u32(found.wrapping_add(0x18)) != 0 {
            return 0;
        }
        let current = lf_checker_rt::callee_cdecl!(1, u32, 0);
        if read_u32(current.wrapping_add(0x0B30)) == vehicle {
            let busy = lf_checker_rt::callee_thiscall!(3, u32, vehicle);
            if busy & 0xFF != 0 {
                return 0;
            }
            let mission = lf_checker_rt::callee_thiscall!(4, u32, vehicle);
            if read_u32(mission.wrapping_add(0x04C8)) != 0 {
                return 0;
            }
        }
        let state = read_u32(read_u32(ped.wrapping_add(0x21C)).wrapping_add(0x12C));
        let aboard = read_u32(ped.wrapping_add(0x0B30));
        if state == 0x12 {
            return (aboard != vehicle) as u32;
        }
        if state != 2 {
            return 0;
        }
        (aboard == vehicle) as u32
    }
});
