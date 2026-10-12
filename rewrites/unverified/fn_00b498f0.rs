// original: 0x00B498F0 CEventVehicleDamageWeapon::vf8

/// Whether the vehicle-damage-weapon event affects a ped: after the usual seat and task checks the event must name a vehicle record, a bike-kind record must fail the compatibility helper, the group helper must decline, and the ped must sit in the event's vehicle.
///
/// `this` points to the event, `ped` to the candidate ped. Outgoing helper
/// calls are intercepted by the contract; all comparisons are unsigned.
/// Returns 1 when the event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b498f0(this: u32, ped: u32) -> u32 {
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
        let record = read_u32(this.wrapping_add(0x1C));
        if record == 0 {
            return 0;
        }
        if read_u32(record.wrapping_add(0x28)) & 0x3C0 == 0xC0 {
            let compatible = lf_checker_rt::callee_thiscall!(1, u32, tasks, record);
            if compatible & 0xFF != 0 {
                return 0;
            }
        }
        let grouped = lf_checker_rt::callee_thiscall!(2, u32, tasks.wrapping_add(0x84), this);
        if grouped & 0xFF != 0 {
            return 0;
        }
        let vehicle = read_u32(this.wrapping_add(0x18));
        if vehicle == 0 {
            return 0;
        }
        (read_u32(ped.wrapping_add(0x0B30)) == vehicle) as u32
    }
});
