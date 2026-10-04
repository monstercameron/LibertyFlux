// original: 0x00B6DBB0 task_dtor_rel34_min_tail2
/// Destroy a task: release the owned handle, stamp the vtable, then run
/// its own base destructor.
export!(thiscall, rw_00b6dbb0(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x34)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB206C);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x34)) as u32);
        }
        callee_thiscall!(4, u32, this as u32)
    }
});
