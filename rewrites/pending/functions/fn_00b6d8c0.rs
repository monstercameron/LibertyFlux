// original: 0x00B6D8C0 task_dtor_rel1c_unlink_only
/// Destroy a task: release the owned handle, stamp the vtable, unlink the
/// link when set, then run the base destructor.
export!(thiscall, rw_00b6d8c0(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x1c)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1DAC);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x1c)) as u32);
        }
        let link = *((this.add(0x18)) as *mut u32);
        if link != 0 {
            callee_thiscall!(3, u32, link, this as u32);
            *((this.add(0x18)) as *mut u32) = 0;
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
