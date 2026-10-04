// original: 0x00d1f5b0 CTaskComplexSeekCover::vf21
// Allocates a small task node from the task pool, runs its constructor,
// then fills in the vtable, the argument word and zeroed tail fields.
// Returns the node, or null when allocation fails.
export!(thiscall, rw_00d1f5b0(_this: u32, arg: u32) -> u32 {
    unsafe {
        let mgr = *(global::<u32>(0x0167E2A0));
        let node = callee_thiscall!(1, u32, mgr);
        if node == 0 {
            return 0;
        }
        callee_thiscall!(2, u32, node);
        *(node as *mut u32) = relocated(0x00EB391C);
        *((node.wrapping_add(0x14)) as *mut u32) = arg;
        *((node.wrapping_add(0x18)) as *mut u8) = 0;
        *((node.wrapping_add(0x1C)) as *mut u32) = 0;
        node
    }
});
