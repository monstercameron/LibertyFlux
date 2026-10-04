// original: 0x005dcdb0 CTaskSimpleNone::vf1
/// Clone an empty task: allocate from the pool, run the trivial constructor,
/// stamp the vtable. Returns the new task or null.
export!(thiscall, rw_005dcdb0(this_ptr: *mut u8) -> u32 {
    unsafe {
        let _ = this_ptr;
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        callee_thiscall!(2, u32, new);
        *(new as *mut u32) = relocated(0xEB38C4);
        new
    }
});
