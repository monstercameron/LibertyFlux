// original: 0x005dd160 CTaskSimpleAssessInjuredPed::~CTaskSimpleAssessInjuredPed
/// Member destructor of an assess task: detach the target, shut down the
/// timer, tear down the position block, then tail into the base destructor.
export!(thiscall, rw_005dd160(this_ptr: *mut u8) -> u32 {
    unsafe {
        *(this_ptr as *mut u32) = relocated(0xFE0C5C);
        let target = *((this_ptr as *const u8).add(0x1c) as *const u32);
        if target != 0 {
            callee_thiscall!(1, u32, target, (this_ptr as u32).wrapping_add(0x1c));
            *((this_ptr as *mut u8).add(0x1c) as *mut u32) = 0;
        }
        let timer = *((this_ptr as *const u8).add(0x20) as *const u32);
        if timer != 0 {
            callee_thiscall!(2, u32, timer, this_ptr as u32);
            callee_thiscall!(3, u32,
                *((this_ptr as *const u8).add(0x20) as *const u32), 0xC47A0000);
            *((this_ptr as *mut u8).add(0x20) as *mut u32) = 0;
        }
        callee_thiscall!(4, u32, (this_ptr as u32).wrapping_add(0x34));
        callee_thiscall!(5, u32, this_ptr as u32)
    }
});
