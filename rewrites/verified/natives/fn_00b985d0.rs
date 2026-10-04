// original: 0x00b985d0 IS_IN_CAR_FIRE_BUTTON_PRESSED
/// Native handler `IS_IN_CAR_FIRE_BUTTON_PRESSED`: test the in-car fire button: no script args, return the engine's 0/1.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00b985d0(ctx: u32) -> u32 {
    unsafe {
        let ans = callee_cdecl!(1, u32,);
        // The engine returns a byte-wide boolean; the handler stores it
        // zero-extended to 32 bits.
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
