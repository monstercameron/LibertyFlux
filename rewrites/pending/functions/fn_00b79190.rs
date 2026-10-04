// original: 0x00b79190 task_allocator_init
/// Allocate and set up the global task allocator.
///
/// Allocates `0x1c` bytes; on success runs the allocator setup routine on
/// the block and publishes the result in the global slot, otherwise
/// publishes null. Returns the published value.
export!(cdecl, rw_00b79190() -> u32 {
    let p: u32 = callee_cdecl!(1, u32, 0x1cu32);
    unsafe {
        if p != 0 {
            let r: u32 = callee_thiscall!(2, u32, p, 0x4b0u32, relocated(0x00EB2B68), 0x110u32);
            *global::<u32>(0x0167E2A0) = r;
            r
        } else {
            *global::<u32>(0x0167E2A0) = 0;
            0
        }
    }
});
