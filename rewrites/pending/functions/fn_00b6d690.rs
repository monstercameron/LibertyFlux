// original: 0x00B6D690 task_ctor_vec_copy
/// Construct a task object: run the base constructor, store the incoming
/// handle, stamp the vtable, copy three words from the source vector, bind
/// the trailing argument, and register each non-zero handle.
export!(thiscall, rw_00b6d690(this: *mut u8, a0: u32, src: *const u32, a2: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *((this.add(0x14)) as *mut u32) = a0;
        *(this as *mut u32) = relocated(0xEB1FBC);
        *this.add(0x18) = 1;
        *((this.add(0x20)) as *mut u32) = *src;
        *((this.add(0x24)) as *mut u32) = *src.add(1);
        *((this.add(0x28)) as *mut u32) = *src.add(2);
        *((this.add(0x30)) as *mut u32) = a2;
        if a0 != 0 {
            callee_thiscall!(2, u32, a0, (this.add(0x14)) as u32);
        }
        if a2 != 0 {
            callee_thiscall!(2, u32, a2, (this.add(0x30)) as u32);
        }
        this as u32
    }
});
