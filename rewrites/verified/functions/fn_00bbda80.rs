// original: 0x00bbda80 NativeImpl_TASK_COWER
/// Queue a cower task for a ped.
///
/// Does nothing and returns the gate answer when the start-up gate is set.
/// Otherwise allocates a task node; when the allocator reports empty, assigns
/// the empty task with kind 6. When a node is available, builds the cower
/// task from the "Cower" parameters, installs the cower function table on the
/// node and assigns it with kind 6. Returns the assign call's answer.
export!(cdecl, rw_00bbda80(handle: u32) -> u32 {
    unsafe {
        let gate: u32 = callee_cdecl!(1, u32,);
        if (gate & 0xFF) != 0 {
            return gate;
        }
        let node: u32 = callee_thiscall!(2, u32, *global::<u32>(0x167E2A0));
        if node == 0 {
            return callee_cdecl!(4, u32, handle, 0, 6);
        }
        let _: u32 = callee_thiscall!(3, u32, node, 0, 3, 0x40800000u32,
            0x19C, relocated(0xFE15DC), 0, 0x3F800000u32, 0);
        *(node as *mut u32) = relocated(0xEB7374);
        callee_cdecl!(4, u32, handle, node, 6)
    }
});
