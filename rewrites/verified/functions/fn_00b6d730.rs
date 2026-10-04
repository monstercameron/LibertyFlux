// original: 0x00B6D730 task_dtor_rel1c_float_f_c080
/// Destroy a task: release the owned handle, stamp the vtable, retire the
/// link with a float update and an unlink, flag the peer, then run the base
/// destructor.
export!(thiscall, rw_00b6d730(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x1c)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1A94);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x1c)) as u32);
        }
        let link = *((this.add(0x18)) as *mut u32);
        if link != 0 {
            callee_thiscall!(2, u32, link, 0xC0800000);
            callee_thiscall!(3, u32, link, this as u32);
            *((link.wrapping_add(4)) as *mut u32) |= 0x4000;
            *((this.add(0x18)) as *mut u32) = 0;
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
