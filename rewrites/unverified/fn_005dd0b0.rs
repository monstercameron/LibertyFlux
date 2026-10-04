// original: 0x005dd0b0 CTaskSimpleAssessInjuredPed::CTaskSimpleAssessInjuredPed
/// Construct an assess-injured-ped task in place: base-construct, set the
/// state bit, store the target handle, blank the timers, attach to the
/// target when set. Returns `this`.
export!(thiscall, rw_005dd0b0(this_ptr: *mut u8, target: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr as u32);
        let b = *this_ptr.add(0x14);
        *this_ptr.add(0x14) = (b & 0xfd) | 1;
        *(this_ptr as *mut u32) = relocated(0xFE0C5C);
        *((this_ptr as *mut u8).add(0x18) as *mut u32) = 0xFFFFFFFF;
        *((this_ptr as *mut u8).add(0x1c) as *mut u32) = target;
        *((this_ptr as *mut u8).add(0x20) as *mut u32) = 0;
        *((this_ptr as *mut u8).add(0x24) as *mut u32) = 0;
        *((this_ptr as *mut u8).add(0x28) as *mut u32) = 0;
        *((this_ptr as *mut u8).add(0x2c) as *mut u32) = 0;
        *((this_ptr as *mut u8).add(0x30) as *mut u16) = 0;
        callee_thiscall!(2, u32, (this_ptr as u32).wrapping_add(0x34));
        if target != 0 {
            callee_thiscall!(3, u32, target, (this_ptr as u32).wrapping_add(0x1c));
        }
        *this_ptr.add(0x30) = 0;
        this_ptr as u32
    }
});
