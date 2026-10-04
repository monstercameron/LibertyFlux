// original: 0x008D4CF0 StoreAndAcquireChildRef
/// Under the lock, notifies the current holder through the inner call, stores
/// the new child reference and bumps its reference count when non-null.
export!(thiscall, rw_008D4CF0(this: u32, new_val: u32) -> () {
    unsafe {
        let mut guard = [0u32; 2];
        let gp = guard.as_mut_ptr() as u32;
        callee_thiscall!(0, u32, gp, relocated(0x11736fc));
        callee_thiscall!(1, u32, this);
        *((this) as *mut u32) = new_val;
        if new_val != 0 {
            let rc = (new_val + 0xa) as *mut u16;
            rc.write_unaligned(rc.read_unaligned().wrapping_add(1));
        }
        callee_thiscall!(2, u32, gp);
    }
});
