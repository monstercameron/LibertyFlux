// original: 0x008d5560 emit_fan_pair_block
/// Fan/quad emission helper: constant-block preamble plus a computed 12-word block.
///
/// Calls the blend-block helper with a constant 8-word block, then reads two
/// source floats `x = src[0]`, `y = src[1]` and builds a 12-word emission
/// block from `x`, `y` and the by-value float `f`:
///
/// ```text
/// [x, y-f, x+f, y, x, y+f, x, y-f, x, y+f, x-f, y]
/// ```
///
/// The block is dispatched with `(block, 6, 0, -1.0)` and the dispatch
/// answer is returned (through the stack-cookie check, whose stub answer is
/// scripted identically so the return channel still verifies).
///
/// All float operations are single IEEE `addss`/`subss` steps and match the
/// original bit-exactly.
lf_checker_rt::export!(cdecl, rb120_fn1(src: u32, f: u32) -> u32 {
    const CAL_BLEND: u32 = 1; // blend-block helper (cdecl/3)
    const CAL_DISPATCH: u32 = 2; // block dispatcher (cdecl/4)
    const CAL_COOKIE: u32 = 3; // stack-cookie check (cdecl/0)

    // Constant preamble block (8 words).
    const PRE: [u32; 8] = [
        0xBC23_D70A,
        0xBC23_D70A,
        0x3F81_47AE,
        0xBC23_D70A,
        0x3F81_47AE,
        0x3F81_47AE,
        0xBC23_D70A,
        0x3F81_47AE,
    ];
    const NEG_ONE_BITS: u32 = 0xBF80_0000; // -1.0f dispatch tag

    unsafe {
        // Preamble call first, exactly as in the original.
        lf_checker_rt::callee_cdecl!(
            CAL_BLEND,
            u32,
            PRE.as_ptr() as u32,
            0u32,
            0u32
        );
        let x = *(src as *const f32);
        let y = *((src + 4) as *const f32);
        let ff = f32::from_bits(f);
        let ymf = y - ff;
        let xpf = x + ff;
        let ypf = y + ff;
        let xmf = x - ff;
        let block: [u32; 12] = [
            x.to_bits(),
            ymf.to_bits(),
            xpf.to_bits(),
            y.to_bits(),
            x.to_bits(),
            ypf.to_bits(),
            x.to_bits(),
            ymf.to_bits(),
            x.to_bits(),
            ypf.to_bits(),
            xmf.to_bits(),
            y.to_bits(),
        ];
        let r = lf_checker_rt::callee_cdecl!(
            CAL_DISPATCH,
            u32,
            block.as_ptr() as u32,
            6u32,
            0u32,
            NEG_ONE_BITS
        );
        lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        r
    }
});
