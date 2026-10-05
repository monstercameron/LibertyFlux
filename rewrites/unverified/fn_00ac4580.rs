// original: 0x00AC4580 stream_submit_sized_copy (proposed)

/// Submit a sized copy with dimensions chosen by two probes.
///
/// The original calls the probe callee twice, picking each dimension from
/// its global pair by the low byte of the probe answer, then calls the
/// submit callee with the nine stack words plus a four-word descriptor
/// (two zero words, the dimensions as floats) (cdecl, nine words: five
/// integers interleaved with three floats and two trailing integers).
/// The callee's writes through the descriptor land in uncompared scratch.
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC4580(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32,
) -> u32 {
    unsafe {
        const PROBE_A: u32 = 1;
        const PROBE_B: u32 = 2;
        const SUBMIT: u32 = 3;
        const DIM0_LO: u32 = 0x0105C880;
        const DIM0_HI: u32 = 0x0105C87C;
        const DIM1_LO: u32 = 0x0105C884;
        const DIM1_HI: u32 = 0x0105C888;
        let pa = lf_checker_rt::callee_cdecl!(PROBE_A, u32,) as u8;
        let d0 = lf_checker_rt::relocated(if pa != 0 { DIM0_HI } else { DIM0_LO });
        let dim0 = (d0 as *const u32).read_unaligned();
        let pb = lf_checker_rt::callee_cdecl!(PROBE_B, u32,) as u8;
        let d1 = lf_checker_rt::relocated(if pb != 0 { DIM1_HI } else { DIM1_LO });
        let dim1 = (d1 as *const u32).read_unaligned();
        let mut desc = [0u32, 0u32, (dim1 as i32) as f32 as u32, (dim0 as i32) as f32 as u32];
        // NOTE: bits produced through the same int->float conversion.
        desc[2] = f32::to_bits(dim1 as i32 as f32);
        desc[3] = f32::to_bits(dim0 as i32 as f32);
        lf_checker_rt::callee_cdecl!(
            SUBMIT, u32, a0, a1, a2, a3, a4, a5, a6, a7, a8, &mut desc as *mut u32 as u32
        );
        0
    }
});
