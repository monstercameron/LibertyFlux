// original: 0x008AB1A0 audio_backend_start
/// Start the audio backend: run two parameter-less init steps, then invoke
/// the six-argument bring-up routine with the live backend configuration
/// words and the second step's result. Returns 1 in the low byte.
export!(cdecl, rw_008AB1A0() -> u32 {
    unsafe {
        let g = |va: u32| *(global::<u32>(va) as *const u32);
        let _first: u32 = callee_cdecl!(1, u32,);
        let second: u32 = callee_cdecl!(2, u32,);
        let r: u32 = callee_cdecl!(3, u32,
            second,
            g(0x0115DEB4),
            0xF0,
            g(0x0115DEBC),
            g(0x0115DEB8),
            g(0x0115DEC0));
        (r & !0xFF) | 1
    }
});
