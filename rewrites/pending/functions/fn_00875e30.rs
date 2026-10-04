// original: 0x00875e30 rage::crmtNodeExtrapolate::vf1
/// Tear down an extrapolate node: close children, uninit, clear the links.
///
/// thiscall/0 (`rage::crmtNodeExtrapolate::vf1`). Closes each attached
/// child whose ownership flag is set, runs the two shared teardown steps,
/// then clears the link fields (the head link only when one is set).
export!(thiscall, rw_00875e30(this: *mut u8) -> () {
    unsafe {
        let first = *(this.add(0x24) as *const u32);
        if first != 0 && *(this.add(0x2c)) != 0 {
            let vtable = *(first as *const u32);
            let close: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*(vtable as *const u32) as usize);
            close(first, 1);
        }
        *(this.add(0x24) as *mut u32) = 0;
        let second = *(this.add(0x28) as *const u32);
        if second != 0 && *(this.add(0x2d)) != 0 {
            let vtable = *(second as *const u32);
            let close: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*(vtable as *const u32) as usize);
            close(second, 1);
        }
        *(this.add(0x28) as *mut u32) = 0;
        callee_thiscall!(3, u32, this as u32);
        callee_thiscall!(4, u32, this as u32);
        *(this.add(0x0c) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        if *(this.add(8) as *const u32) != 0 {
            *(this.add(8) as *mut u32) = 0;
        }
    }
});
