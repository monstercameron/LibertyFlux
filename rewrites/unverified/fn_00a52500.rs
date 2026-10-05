// original: 0x00a52500 vehicle_probe_wrapper (proposed)

/// Probe through the built struct, returning the output or the default.
///
/// Builds a 112-byte probe struct in scratch (zeroed, then the two argument
/// floats, three global floats, two constants and the small fields at their
/// exact offsets) and calls the probe callee with pointers to its base and
/// to +16 plus the constant, 0, 6 and the third argument (the argument
/// floats live only in the struct). A nonzero answer returns the callee's
/// output word (at +40,
/// loaded float-extended); zero returns the global default float. Both
/// float loads quiet a signalling NaN exactly like fld. The scratch
/// addresses are skipped and the struct bytes snapshotted (see contract);
/// the stack-pointer check is off because this callee-cleanup-shaped
/// function returns plain (see contract). Thiscall, three stack words
/// (`this` unread), one callee, float in ST0.
lf_checker_rt::export!(thiscall, rw_00a52500(this: u32, a0: u32, a1: u32, a2: u32) -> f32 {
    unsafe {
        const CALLEE: u32 = 1;
        const G1: u32 = 0x01b4b328;
        const G2: u32 = 0x01b4b320;
        const G3: u32 = 0x01b4b324;
        const G4: u32 = 0x00fe8b38;
        const C_A: u32 = 0xc47a0000;
        const C_B: u32 = 0x447a0000;
        const OUT_OFF: u32 = 40;
        const FRAMEA_OFF: u32 = 16;
        let _ = this;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// What `fld` leaves in ST0 for these bits (SNaN quieted).
        fn quieted(bits: u32) -> f32 {
            let q = if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
                bits | 0x0040_0000
            } else {
                bits
            };
            f32::from_bits(q)
        }
        let g1 = rd32(lf_checker_rt::relocated(G1));
        let g2 = rd32(lf_checker_rt::relocated(G2));
        let g3 = rd32(lf_checker_rt::relocated(G3));
        let g4 = rd32(lf_checker_rt::relocated(G4));
        let mut st = [0u32; 28];
        st[0] = a0;
        st[1] = a1;
        st[2] = C_B;
        st[8] = g2;
        st[9] = g3;
        st[10] = g1;
        st[12] = g2;
        st[13] = g3;
        st[14] = g1;
        st[16] = g2;
        st[17] = g3;
        st[18] = g1;
        st[23] = 0xffff;
        let base = st.as_mut_ptr() as u32;
        let frame_b = base;
        let frame_a = base.wrapping_add(FRAMEA_OFF);
        let r: u32 = lf_checker_rt::callee_cdecl!(CALLEE, u32, frame_b, C_A, 0, frame_a, 6, a2);
        if (r & 0xff) != 0 {
            quieted(rd32(frame_b.wrapping_add(OUT_OFF)))
        } else {
            quieted(g4)
        }
    }
});
