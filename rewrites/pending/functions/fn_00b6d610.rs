// original: 0x00B6D610 task_ctor_flag_u8
/// Construct a task object: run the base constructor, store the incoming
/// flag byte, stamp the vtable, and initialise the state words.
export!(thiscall, rw_00b6d610(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *this.add(0x14) = (flag & 0xFF) as u8;
        *(this as *mut u32) = relocated(0xEB20C4);
        *((this.add(0x18)) as *mut u32) = 0xFFFFFFFF;
        *((this.add(0x1c)) as *mut u32) = 0;
        this as u32
    }
});
