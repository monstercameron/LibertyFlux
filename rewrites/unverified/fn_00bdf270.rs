// original: 0x00BDF270 CTaskSimpleOpenDoor::CTaskSimpleOpenDoor

/// Initializes an open-door task with its relocated vtable, default counters,
/// and the invalid-node sentinel. When a resource pointer is supplied, stores
/// it in the node field, asks the attach helper to update that field, then
/// copies byte 0x22b from the attached resource into task offset 0x20. The
/// thiscall receiver is returned in EAX and one stack argument is popped.
lf_checker_rt::export!(thiscall, rw_bdf270(this: u32, resource: u32) -> u32 {
    unsafe {
        const BASE_TASK_CTOR: u32 = 1;
        const ATTACH_FREE_NODE: u32 = 2;
        const OPEN_DOOR_VTABLE: u32 = 0x00EB93FC;
        const NODE_POINTER: u32 = 0x14;
        const NODE_STATE: u32 = 0x20;
        const NODE_SENTINEL: u32 = 0x1c;
        const NODE_FLAGS: u32 = 0x18;
        const NODE_BYTE: u32 = 0x24;
        const RESOURCE_STATE: u32 = 0x22b;
        const INVALID_NODE: u32 = u32::MAX;

        let _ = lf_checker_rt::callee_thiscall!(BASE_TASK_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(OPEN_DOOR_VTABLE));
        (this.wrapping_add(NODE_POINTER) as *mut u32).write_unaligned(0);
        (this.wrapping_add(NODE_FLAGS) as *mut u32).write_unaligned(0);
        (this.wrapping_add(NODE_SENTINEL) as *mut u32).write_unaligned(INVALID_NODE);
        (this.wrapping_add(NODE_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(NODE_BYTE) as *mut u8).write(0);

        if resource != 0 {
            let node_pointer = this.wrapping_add(NODE_POINTER);
            (node_pointer as *mut u32).write_unaligned(resource);
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_FREE_NODE, u32, resource, node_pointer);
            let attached_resource = (node_pointer as *const u32).read_unaligned();
            let state = (attached_resource.wrapping_add(RESOURCE_STATE) as *const u8).read();
            (this.wrapping_add(NODE_STATE) as *mut u32).write_unaligned(u32::from(state));
        }
        this
    }
});
