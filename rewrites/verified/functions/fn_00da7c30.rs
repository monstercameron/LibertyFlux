// original: 0x00DA7C30 CTaskComplexSmartFleePoint::vf19

/// Copy the ped's embedded point into this task, then request the special
/// flee task when the ped flag and special-object reference are both set.
/// Otherwise ask the distance helper for its low-byte state, store it at
/// `+0x68`, and request the default task. The nested point pointer is at
/// ped offset `+0x20`; its four words are copied without float arithmetic.
lf_checker_rt::export!(thiscall, rw_00DA7C30(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_POINT: u32 = 0x20;
        const POINT_X: u32 = 0x30;
        const TASK_POINT: u32 = 0x20;
        const PED_FLAGS: u32 = 0x26C;
        const PED_SPECIAL_OBJECT: u32 = 0xB30;
        const SPECIAL_BIT: u8 = 0x04;
        const DISTANCE_STATE: u32 = 0x68;
        const CODE_SPECIAL: u32 = 0x2CA;
        const CODE_DEFAULT: u32 = 0x3AE;
        const TASK_FACTORY: u32 = 1;
        const DISTANCE_HELPER: u32 = 2;

        let point = (ped.wrapping_add(PED_POINT) as *const u32).read_unaligned();
        let x = (point.wrapping_add(POINT_X) as *const u32).read_unaligned();
        let y = (point.wrapping_add(POINT_X + 4) as *const u32).read_unaligned();
        let z = (point.wrapping_add(POINT_X + 8) as *const u32).read_unaligned();
        let heading = (point.wrapping_add(POINT_X + 12) as *const u32).read_unaligned();
        (this.wrapping_add(TASK_POINT) as *mut u32).write_unaligned(x);
        (this.wrapping_add(TASK_POINT + 4) as *mut u32).write_unaligned(y);
        (this.wrapping_add(TASK_POINT + 8) as *mut u32).write_unaligned(z);
        (this.wrapping_add(TASK_POINT + 12) as *mut u32).write_unaligned(heading);

        let flags = (ped.wrapping_add(PED_FLAGS) as *const u8).read();
        let special_object = (ped.wrapping_add(PED_SPECIAL_OBJECT) as *const u32).read_unaligned();
        if flags & SPECIAL_BIT != 0 && special_object != 0 {
            lf_checker_rt::callee_thiscall!(TASK_FACTORY, u32, this, CODE_SPECIAL, ped)
        } else {
            let state = lf_checker_rt::callee_thiscall!(DISTANCE_HELPER, u32, this, ped) as u8;
            (this.wrapping_add(DISTANCE_STATE) as *mut u8).write(state);
            lf_checker_rt::callee_thiscall!(TASK_FACTORY, u32, this, CODE_DEFAULT, ped)
        }
    }
});
