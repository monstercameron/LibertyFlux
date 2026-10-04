// original: 0x00ae2460 ui_listener_register
/// Register a listener object on the global listener list.
///
/// A null object is ignored. Otherwise the object's threshold word at `+0x50`
/// is set to the constant `751.0f` bits, a list node is resolved through the
/// allocator, and the node (holding the object and the previous head) becomes
/// the new head. A null node faults on both sides, exactly like the original.
/// Returns nothing meaningful.
export!(cdecl, rw_00ae2460(obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        ((obj + 0x50) as *mut u32).write(0x443B8000);
        let alloc = *global::<u32>(0x12B4164);
        let node = callee_thiscall!(1, u32, alloc);
        if node == 0 {
            let head = *global::<u32>(0x15C3BA8);
            ((node + 4) as *mut u32).write(head);
            *global::<u32>(0x15C3BA8) = node;
        } else {
            (node as *mut u32).write(obj);
            ((node + 4) as *mut u32).write(*global::<u32>(0x15C3BA8));
            *global::<u32>(0x15C3BA8) = node;
        }
        0
    }
});
