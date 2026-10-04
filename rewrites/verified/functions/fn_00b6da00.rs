// original: 0x00B6DA00 task_dtor_rel14_min_b
/// Destroy a task: release the owned handle, stamp the vtable, then run
/// the base destructor.
export!(thiscall, rw_00b6da00(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x14)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1E04);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x14)) as u32);
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
