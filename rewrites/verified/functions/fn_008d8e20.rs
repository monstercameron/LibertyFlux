// original: 0x008d8e20 NativeImpl_FORCE_LOADING_SCREEN
/// Store a mode byte into this object, notifying when nonzero.
///
/// Writes the low byte of the argument at 0xFCB; a nonzero value also
/// triggers the fixed notify call with this object in ECX.
export!(thiscall, rw_008d8e20(this: u32, arg: u32) -> u32 {
    unsafe {
        /// Object field offset receiving the mode byte.
        const FIELD: usize = 0xFCB;
        let b = (arg & 0xFF) as u8;
        (this as *mut u8).add(FIELD).write(b);
        if b != 0 {
            let _: u32 = callee_thiscall!(1, u32, this);
        }
        0
    }
});
