// original: 0x008d56c0 emit_centered_pair_block
/// Centered-pair emission helper: preamble plus two dispatched blocks.
///
/// Calls the blend-block helper with the same constant 8-word block as its
/// sibling, reads `x = src[0]`, `y = src[1]`, stages `[x, y, x]` and calls
/// the n-block dispatcher with `(stage, f, scratch, ecx, -1.0)` where
/// `scratch` is the third incoming stack word (modelled as an argument) and
/// the incoming-ECX word is passed through opaquely (skipped in the call
/// comparison: no Rust code can observe incoming ECX in a cdecl function).
/// Then builds the 6-word centered block `[x, y, x+f, 1.0, x-f, 1.0]`,
/// dispatches it with `(block, 1, 0, -1.0)` and returns the answer through
/// the stack-cookie check (same shared-script return trick as fn1).
lf_checker_rt::export!(cdecl, rb120_fn2(src: u32, f: u32, scratch: u32) -> u32 {
    const CAL_BLEND: u32 = 1;
    const CAL_NBLOCK: u32 = 2;
    const CAL_DISPATCH: u32 = 3;
    const CAL_COOKIE: u32 = 4;
    const PRE: [u32; 8] = [
        0xBC23_D70A, 0xBC23_D70A, 0x3F81_47AE, 0xBC23_D70A,
        0x3F81_47AE, 0x3F81_47AE, 0xBC23_D70A, 0x3F81_47AE,
    ];
    const NEG_ONE_BITS: u32 = 0xBF80_0000;
    const ONE_BITS: u32 = 0x3F80_0000;
    unsafe {
        lf_checker_rt::callee_cdecl!(CAL_BLEND, u32, PRE.as_ptr() as u32, 0u32, 0u32);
        let x = *(src as *const f32);
        let y = *((src + 4) as *const f32);
        let ff = f32::from_bits(f);
        let stage: [u32; 3] = [x.to_bits(), y.to_bits(), x.to_bits()];
        lf_checker_rt::callee_cdecl!(
            CAL_NBLOCK, u32, stage.as_ptr() as u32, f, scratch, 0u32, NEG_ONE_BITS
        );
        let xpf = x + ff;
        let xmf = x - ff;
        let block: [u32; 6] = [
            x.to_bits(), y.to_bits(), xpf.to_bits(), ONE_BITS, xmf.to_bits(), ONE_BITS,
        ];
        let r = lf_checker_rt::callee_cdecl!(
            CAL_DISPATCH, u32, block.as_ptr() as u32, 1u32, 0u32, NEG_ONE_BITS
        );
        lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        r
    }
});
