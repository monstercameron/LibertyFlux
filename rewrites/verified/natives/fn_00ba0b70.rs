// original: 0x00ba0b70 MP_GET_AMOUNT_OF_VARIATION_COMPONENT
/// Script native `MP_GET_AMOUNT_OF_VARIATION_COMPONENT` (hash 0x54DD6ACF).
///
/// Forwards two script arguments (a character handle and a component
/// /// index) to the engine and stores its full 32-bit answer (a
/// /// variation count) into the return slot.
export!(cdecl, rw_00ba0b70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
