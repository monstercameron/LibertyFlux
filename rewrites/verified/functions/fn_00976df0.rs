// original: 0x00976df0 audio_struct_level_query (proposed)

/// Look up a level through a stack-built query struct, on ST0.
///
/// The fetch callee fills a 22-word struct in the frame (only the pointer at
/// +0x54 is read back); a null pointer gives 1.0, otherwise byte +0x1A of the
/// target scales by the read-only constant. The trailing call is the CRT
/// security-cookie check, which preserves all registers. Null compare only.
/// Original: 0x00976DF0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00976df0(arg: u32) -> f32 {
    unsafe {
        const FETCH: u32 = 1;
        const COOKIE: u32 = 2;
        const RES_OFF: u32 = 0x58;
        const LEVEL_OFF: u32 = 0x1A;
        const SCALE: f32 = f32::from_bits(0x3c23d70a);
        let mut buf = [0u32; 23];
        lf_checker_rt::callee_stdcall!(
            FETCH,
            u32,
            &mut buf as *mut u32 as u32,
            arg
        );
        let p = buf[(RES_OFF / 4) as usize];
        let f = if p == 0 {
            1.0
        } else {
            let b = ((p.wrapping_add(LEVEL_OFF)) as *const u8).read() as f32;
            core::hint::black_box(b) * core::hint::black_box(SCALE)
        };
        lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        f
    }
});
