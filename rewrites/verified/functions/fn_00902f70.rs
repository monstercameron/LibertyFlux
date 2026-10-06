// original: 0x00902f70 input_emit_dual_frame (proposed)
/// Emit one dual frame: stage constants, run setup twice, forward pointers.
///
/// Eight constant words are staged and the setup callee runs on `(0xA, 0)`
/// then `(7, 1)`. When the gate word is set the probe callee (a thiscall on
/// the static gate object) runs and the emit callee runs on five staged-word
/// pointers. The cookie-check callee always runs last and its answer is
/// returned. Cdecl with no arguments.
export!(cdecl, rw_00902f70() -> u32 {
    unsafe {
        /// Gate word, also the static probe object (file VA).
        const GATE: u32 = 0x010344D4;
        /// Eight staged constant words.
        const WORDS: [u32; 8] = [
            0x3CF5C28F, 0x3F7851EC, 0x3CF5C28F, 0x3CF5C28F, 0x3F7851EC, 0x3F7851EC,
            0x3F7851EC, 0x3CF5C28F,
        ];
        /// Temp-slot finals the original leaves below the staged words.
        const TEMP0: u32 = 0x3F7851EC;
        const TEMP1: u32 = 0x3CF5C28F;
        const SETUP_ID: u32 = 1;
        const PROBE_ID: u32 = 2;
        const EMIT_ID: u32 = 4;
        const COOKIE_ID: u32 = 5;
        let mut buf = [0u32; 11];
        buf[0] = TEMP1;
        buf[1] = TEMP0;
        buf[2] = TEMP1;
        buf[3..11].copy_from_slice(&WORDS);
        let base = buf.as_mut_ptr().wrapping_add(1) as u32;
        let _: u32 = callee_cdecl!(SETUP_ID, u32, 0xAu32, 0u32);
        let _: u32 = callee_cdecl!(SETUP_ID, u32, 7u32, 1u32);
        if (global::<u32>(GATE)).read_unaligned() == 0 {
            return callee_cdecl!(COOKIE_ID, u32,);
        }
        let _: u32 = callee_thiscall!(PROBE_ID, u32, relocated(GATE));
        // The original stores -1 into its temp slot just before the call.
        buf[1] = 0xFFFFFFFF;
        let q0 = base.wrapping_add(0x08);
        let q1 = base.wrapping_add(0x10);
        let q2 = base.wrapping_add(0x18);
        let q3 = base.wrapping_add(0x20);
        let _: u32 = callee_cdecl!(EMIT_ID, u32, q0, q1, q2, q3, base);
        callee_cdecl!(COOKIE_ID, u32,)
    }
});
