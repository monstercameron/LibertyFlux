// original: 0x00d1efd0 CTaskComplexSeekCoverShooting::vf19
// Builds the task's follow-up action for the given object. When the
// object's mode bit is set, allocates and minimally constructs a fresh
// node and returns it (or null when allocation fails). Otherwise runs the
// shared solver into a scratch buffer and dispatches through this task's
// vtable, returning that answer.
export!(thiscall, rw_00d1efd0(this_ptr: u32, obj: u32) -> u32 {
    unsafe {
        if (*((obj.wrapping_add(0x26C)) as *const u8) & 4) != 0 {
            let mgr = *(global::<u32>(0x0167E2A0));
            let node = callee_thiscall!(1, u32, mgr);
            if node == 0 {
                return 0;
            }
            callee_thiscall!(2, u32, node);
            *(node as *mut u32) = relocated(0x00E89044);
            *((node.wrapping_add(0x14)) as *mut u32) = 0;
            *((node.wrapping_add(0x18)) as *mut u16) = 1;
            *((node.wrapping_add(0x1A)) as *mut u8) = 1;
            return node;
        }
        let mut scratch = [0u32; 3];
        callee_thiscall!(3, u32, this_ptr, obj, scratch.as_mut_ptr() as u32);
        let vtable = *(this_ptr as *const u32);
        let target = *((vtable.wrapping_add(0x54)) as *const u32);
        let dispatch: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        dispatch(this_ptr, 0x3E8)
    }
});
