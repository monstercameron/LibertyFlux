// original: 0x00902e60 input_emit_gated_frame (proposed)
/// Emit one gated frame: prepare constants, then forward frame pointers.
///
/// When the outer gate word is set, eight constant words are staged and the
/// setup callee runs on `(7, 1)`; when the inner gate word is then also set
/// the probe callee (a thiscall on the static inner object) runs, the
/// gather callee runs on `(base, 3, flagbyte)` where `base` points at the
/// staged words, and the emit callee runs on four staged-word pointers plus
/// the gather answer. The cookie-check callee always runs last and its
/// answer is returned. Cdecl with no arguments.
export!(cdecl, rw_00902e60() -> u32 {
    unsafe {
        /// Outer and inner gate words (file VAs).
        const GATE_OUTER: u32 = 0x01160C9C;
        const GATE_INNER: u32 = 0x010344D0;
        /// Static inner object and flag byte (file VAs).
        const INNER: u32 = 0x010344D0;
        const FLAGB: u32 = 0x010344D8;
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
        const GATHER_ID: u32 = 3;
        const EMIT_ID: u32 = 4;
        const COOKIE_ID: u32 = 5;
        if (global::<u32>(GATE_OUTER)).read_unaligned() == 0 {
            return callee_cdecl!(COOKIE_ID, u32,);
        }
        let mut buf = [0u32; 11];
        buf[0] = TEMP1;
        buf[1] = TEMP0;
        buf[2] = TEMP1;
        buf[3..11].copy_from_slice(&WORDS);
        let base = buf.as_mut_ptr().wrapping_add(1) as u32;
        let _: u32 = callee_cdecl!(SETUP_ID, u32, 7u32, 1u32);
        if (global::<u32>(GATE_INNER)).read_unaligned() == 0 {
            return callee_cdecl!(COOKIE_ID, u32,);
        }
        let _: u32 = callee_thiscall!(PROBE_ID, u32, relocated(INNER));
        let flag = (global::<u8>(FLAGB)).read() as u32;
        let g: u32 = callee_cdecl!(GATHER_ID, u32, base, 3u32, flag);
        let p0 = base.wrapping_sub(4);
        let p1 = base.wrapping_add(4);
        let p2 = base.wrapping_add(0x0C);
        let p3 = base.wrapping_add(0x20);
        let _: u32 = callee_cdecl!(EMIT_ID, u32, p0, p1, p2, p3, g);
        callee_cdecl!(COOKIE_ID, u32,)
    }
});
