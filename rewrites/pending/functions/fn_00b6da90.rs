// original: 0x00B6DA90 task_dtor_rel24_2c_unlink_only
/// Destroy a task: release both owned handles, stamp the vtable, unlink
/// the link when set, then run the base destructor.
export!(thiscall, rw_00b6da90(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x24)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1EB4);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x24)) as u32);
        }
        let field2 = *((this.add(0x2c)) as *mut u32);
        if field2 != 0 {
            callee_thiscall!(1, u32, field2, (this.add(0x2c)) as u32);
        }
        let link = *((this.add(0x18)) as *mut u32);
        if link != 0 {
            callee_thiscall!(3, u32, link, this as u32);
            *((this.add(0x18)) as *mut u32) = 0;
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
