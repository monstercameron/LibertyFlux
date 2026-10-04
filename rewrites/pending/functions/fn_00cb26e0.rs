// original: 0x00CB26E0 CTaskComplexGoToPointAnyMeans::vf19

/// Pick how this complex task reaches its point: drive, walk, or fail over.
///
/// `this` is the complex task, `ped` the ped. When the task already holds a
/// destination (`this+0x3c` non-null) the ped's vehicle (`ped+0xb30`) and
/// driver flag (`ped+0x26c` bit 2) decide between driving there (0x2c6) and
/// going on foot (0x2de). Without a destination the same vehicle check runs
/// through the drive-ability probe (callee 2, receiver is the vehicle); a
/// ped that can drive gets the drive subtask, anything else the fail-over
/// subtask (0x3ae). Creation goes through the factory (callee 1) as
/// (id, ped) and its result is returned.
///
/// Original: 0x00CB26E0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB26E0(this: u32, ped: u32) -> u32 {
    unsafe {
        const CREATE_SUB: u32 = 1;
        const CAN_DRIVE: u32 = 2;
        const DEST: u32 = 0x3c;
        const PED_VEHICLE: u32 = 0xb30;
        const PED_FLAGS: u32 = 0x26c;
        const DRIVER_BIT: u8 = 4;
        const DRIVE_THERE: u32 = 0x2c6;
        const GO_ON_FOOT: u32 = 0x2de;
        const FAIL_OVER: u32 = 0x3ae;
        let dest = (this.wrapping_add(DEST) as *const u32).read_unaligned();
        let vehicle = (ped.wrapping_add(PED_VEHICLE) as *const u32).read_unaligned();
        if dest != 0 {
            let driving = vehicle != 0
                && (ped.wrapping_add(PED_FLAGS) as *const u8).read_unaligned() & DRIVER_BIT != 0;
            let id = if driving { DRIVE_THERE } else { GO_ON_FOOT };
            return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, id, ped);
        }
        let ok = vehicle != 0
            && (ped.wrapping_add(PED_FLAGS) as *const u8).read_unaligned() & DRIVER_BIT != 0
            && lf_checker_rt::callee_thiscall!(CAN_DRIVE, u32, vehicle, ped) as u8 != 0;
        let id = if ok { DRIVE_THERE } else { FAIL_OVER };
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, id, ped)
    }
});
