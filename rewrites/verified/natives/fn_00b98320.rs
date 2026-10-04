// original: 0x00b98320 GET_MOUSE_SENSITIVITY
/// Script native `GET_MOUSE_SENSITIVITY` (hash 0x41401D46).
///
/// Takes no script arguments: calls the engine worker, which returns a
/// single-precision sensitivity value on the x87 stack, and stores the
/// 32-bit pattern into the return slot. Returns the slot pointer.
export!(cdecl, rw_00b98320(ctx: *const u8) -> u32 {
    unsafe {
        let value: f32 = callee_cdecl!(1, f32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = value.to_bits();
        slot as u32
    }
});
