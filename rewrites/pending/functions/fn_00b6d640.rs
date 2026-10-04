// original: 0x00B6D640 task_ctor_arg_bind
/// Construct a task object: initialise the embedded six-argument member,
/// stamp the vtable, bind the incoming argument, and register it when
/// non-zero.
export!(thiscall, rw_00b6d640(this: *mut u8, arg0: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32, 0xFFFFFFFF, 0xFFFFFFFF, 0x41000000, 0, 0x3F800000, 0);
        *(this as *mut u32) = relocated(0xEB2014);
        *((this.add(0x34)) as *mut u32) = arg0;
        if arg0 != 0 {
            callee_thiscall!(2, u32, arg0, (this.add(0x34)) as u32);
        }
        this as u32
    }
});
