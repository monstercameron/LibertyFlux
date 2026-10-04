// original: 0x008a84a0 audio_apply_gain_matrix
/// Scale one audio frame's channels by a shared gain and mix three sources.
///
/// Multiplies words 0..5 by the gain at word 27 into words 6..11, then for
/// each of the three source words 24..26 stores its product with words 0, 1
/// and 5 into three columns (words 12..14, 15..17, 21..23) and the product
/// of word 4 with the source into a fourth column (words 18..20), keeping
/// the original operand order in every multiply. Returns the input pointer
/// plus 0x6C, the address just past the last source word, which is what the
/// original leaves in EAX.
export!(stdcall, rw_008a84a0(p: *mut f32) -> u32 {
    unsafe {
        let g = *p.add(27);
        *p.add(6) = *p.add(0) * g;
        *p.add(7) = *p.add(1) * g;
        *p.add(8) = *p.add(2) * g;
        *p.add(9) = *p.add(3) * g;
        *p.add(10) = *p.add(4) * g;
        *p.add(11) = *p.add(5) * g;
        let c0 = *p.add(0);
        let c1 = *p.add(1);
        let c4 = *p.add(4);
        let c5 = *p.add(5);
        let s0 = *p.add(24);
        let s1 = *p.add(25);
        let s2 = *p.add(26);
        *p.add(12) = s0 * c0;
        *p.add(13) = s1 * c0;
        *p.add(14) = s2 * c0;
        *p.add(15) = s0 * c1;
        *p.add(16) = s1 * c1;
        *p.add(17) = s2 * c1;
        *p.add(18) = c4 * s0;
        *p.add(19) = c4 * s1;
        *p.add(20) = c4 * s2;
        *p.add(21) = s0 * c5;
        *p.add(22) = s1 * c5;
        *p.add(23) = s2 * c5;
        (p as u32).wrapping_add(0x6C)
    }
});

