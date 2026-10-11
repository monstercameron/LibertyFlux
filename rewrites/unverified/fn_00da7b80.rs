// original: 0x00DA7B80 CTaskComplexFleeAnyMeans::vf19

/// Select a flee-task request code from the ped and task flags, then
/// forward the request to the shared dispatcher.
///
/// `this` is the task object; `ped` is the one stack argument. A live
/// special ped (`+0xB30` nonzero and bit 2 of `+0x26C` set) requests
/// `0x2D4`. Otherwise the task byte at `+0x19` chooses `0x3A0` or
/// `0x38F`. The dispatcher receives `this`, the request code, and `ped`;
/// the first two routes pass both request code and ped. The `0x38F` route
/// passes only the code to a helper that pops two stack words; it consumes
/// the caller return address and faults while unwinding through the ped.
/// The checker proof excludes this faulting route and covers the other two.
lf_checker_rt::export!(thiscall, rw_00DA7B80(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_SPECIAL_OBJECT: u32 = 0xB30;
        const PED_FLAGS: u32 = 0x26C;
        const TASK_MODE: u32 = 0x19;
        const SPECIAL_PED_BIT: u8 = 0x04;
        const CODE_SPECIAL: u32 = 0x2D4;
        const CODE_MODE_SET: u32 = 0x3A0;
        const CODE_MODE_CLEAR: u32 = 0x38F;
        const DISPATCHER_SPECIAL: u32 = 1;
        const DISPATCHER_MODE_SET: u32 = 2;
        const DISPATCHER_MODE_CLEAR: u32 = 3;

        let special_object = (ped.wrapping_add(PED_SPECIAL_OBJECT) as *const u32).read_unaligned();
        let ped_flags = (ped.wrapping_add(PED_FLAGS) as *const u8).read();
        if special_object != 0 && ped_flags & SPECIAL_PED_BIT != 0 {
            lf_checker_rt::callee_thiscall!(DISPATCHER_SPECIAL, u32, this, CODE_SPECIAL, ped)
        } else if (this.wrapping_add(TASK_MODE) as *const u8).read() != 0 {
            lf_checker_rt::callee_thiscall!(DISPATCHER_MODE_SET, u32, this, CODE_MODE_SET, ped)
        } else {
            lf_checker_rt::callee_thiscall!(DISPATCHER_MODE_CLEAR, u32, this, CODE_MODE_CLEAR)
        }
    }
});
