// original: 0x00b06d70 notify_vtable_if_args_differ
/// Notify through the object when two ids differ, otherwise echo the first.
///
/// Compares the two argument words; when they are equal the function does
/// nothing and returns the first one. When they differ it runs the shared
/// reset helper on this object and then the object's second virtual slot,
/// returning that slot's result.
export!(thiscall, rw_00b06d70(this_ptr: u32, a: u32, b: u32) -> u32 {
    unsafe {
        if a == b {
            return a;
        }
        callee_thiscall!(1, u32, this_ptr, 0u32);
        const NOTIFY_SLOT: usize = 4;
        let table = *(this_ptr as *const u32) as usize;
        let target = *((table + NOTIFY_SLOT) as *const u32) as usize;
        let notify: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target);
        notify(this_ptr)
    }
});
