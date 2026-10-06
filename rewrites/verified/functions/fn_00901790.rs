// original: 0x00901790 input_emit_zero_block (proposed)
/// Forward five arguments to the emitter with a zeroed three-word block.
///
/// Builds three zero words on the stack and calls the emitter callee with
/// `(a0, a1, a2, block, a3, a4)` where `block` points at them; returns the
/// callee's answer. Cdecl with five stack words.
export!(cdecl, rw_00901790(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const EMIT_ID: u32 = 1;
        let block = [0u32; 3];
        callee_cdecl!(
            EMIT_ID,
            u32,
            a0,
            a1,
            a2,
            block.as_ptr() as u32,
            a3,
            a4
        )
    }
});
