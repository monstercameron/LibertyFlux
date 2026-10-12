// original: 0x00BB85D0 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>::~CTaskComplexMoveSeekEntity

/// Installs the destructor vtables, detaches the owned timing node when the
/// field at offset 0x20 is non-null, writes the trailing base vtable, then
/// tail-calls the complex-move destructor. The helper receives the node in ECX
/// and the address of the owning field as its stack argument.
lf_checker_rt::export!(thiscall, rw_bb85d0(this: u32) -> u32 {
    unsafe {
        const DETACH_TIMING_NODE: u32 = 1;
        const BASE_MOVE_DTOR: u32 = 2;
        const SEEK_ENTITY_VTABLE: u32 = 0x00EB773C;
        const TASK_COMPONENT_VTABLE: u32 = 0x00EB7794;
        const BASE_COMPONENT_VTABLE: u32 = 0x00EB7688;
        const TIMING_NODE_FIELD: u32 = 0x20;
        const TRAILING_VTABLE_FIELD: u32 = 0x74;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(SEEK_ENTITY_VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_COMPONENT_VTABLE));
        let node_field = this.wrapping_add(TIMING_NODE_FIELD);
        let node = (node_field as *const u32).read_unaligned();
        if node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_TIMING_NODE, u32, node, node_field);
        }
        (this.wrapping_add(TRAILING_VTABLE_FIELD) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_COMPONENT_VTABLE));
        lf_checker_rt::callee_thiscall!(BASE_MOVE_DTOR, u32, this)
    }
});
