// original: 0x0086f540 GET_LATEST_CONSOLE_COMMAND
//
// Debug leftover stub: ignores its arguments and stores a fixed data pointer
// to the return slot addressed by ctx+0. The immediate is a relocated image
// address (it has a base-reloc entry), so it resolves through the worker's
// mapped base. Leaves the return-slot pointer in EAX.
export!(cdecl, rw_0086f540(ctx: *mut u32) -> u32 {
    unsafe {
        let ret_slot = *ctx as *mut u32;
        *ret_slot = relocated(0x00F1_C5FF);
        *ctx
    }
});
