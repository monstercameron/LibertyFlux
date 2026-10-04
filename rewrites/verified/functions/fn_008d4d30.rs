// original: 0x008D4D30 ReleaseAndReplaceChild
/// Under the lock, releases the old child (decrementing its count and freeing
/// through its table when the count hits zero for a freeing type), clears the
/// slot, then converts the incoming argument, wraps it and stores it with a
/// fresh count.
export!(thiscall, rw_008D4D30(this: u32, arg0: u32) -> () {
    unsafe {
        let mut guard = [0u32; 2];
        let gp = guard.as_mut_ptr() as u32;
        callee_thiscall!(0, u32, gp, relocated(0x11736fc));
        let old = *(this as *const u32);
        if old != 0 {
            let rc = ((old + 0xa) as *const u16).read_unaligned();
            if rc != 0 {
                let ty = ((old + 8) as *const u8).read();
                ((old + 0xa) as *mut u16).write_unaligned(rc - 1);
                if rc == 1 && (ty == 2 || ty == 4) {
                    let vtable = *(old as *const u32);
                    let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(*(vtable as *const u32));
                    free_fn(old, 1);
                }
            }
            *(this as *mut u32) = 0;
        }
        if arg0 != 0 {
            let t = callee_cdecl!(2, u32, arg0, 0);
            let new = callee_thiscall!(3, u32, *global::<u32>(0x1bb5554), t);
            *(this as *mut u32) = new;
            if new != 0 {
                let rc = (new + 0xa) as *mut u16;
                rc.write_unaligned(rc.read_unaligned().wrapping_add(1));
            }
        }
        callee_thiscall!(4, u32, gp);
    }
});
