// original: 0x008a8700 audio_voice_pool_init
/// Initialize a voice pool and register it on the current thread.
///
/// Zeroes and stamps the pool's twenty row blocks (five groups of four) with
/// the fixed one/zero pattern, copies three config words into the pool tail,
/// stamps the mode word from the config equality probes (all six matching
/// their reference selects the alternate mode; a mismatch on the first keeps
/// the default; a later mismatch runs the second probe chain, which selects
/// the negative mode only when all five of its comparisons match), marks the
/// pool ready, sets the thread-registered flag in the TLS block named by the
/// index global, and returns that TLS block pointer.
export!(thiscall, rw_008A8700(obj: *mut u8) -> u32 {
    unsafe {
        let f1 = *global::<u32>(VOICE_COPY_1);
        *((obj as *mut u8).add(0x170C) as *mut u32) = f1;
        let f2 = *global::<u32>(VOICE_COPY_2);
        *((obj as *mut u8).add(0x1710) as *mut u32) = f2;
        *((obj as *mut u8).add(0x171C) as *mut u32) = ONE;
        *((obj as *mut u8).add(0x1718) as *mut u32) = NEG_ONE;
        *((obj as *mut u8).add(0x1720) as *mut u32) = NEG_ZERO_BITS;
        let mut row = (obj as u32).wrapping_add(0x18);
        let mut col = (obj as u32).wrapping_add(0x1444);
        let mut blk = (obj as u32).wrapping_add(0x16A4);
        let mut byte_cur = (obj as u32).wrapping_add(0x16F0);
        for _ in 0..5u32 {
            let mut s = blk.wrapping_sub(0xD4);
            for inner in 0..4u32 {
            *((row as *mut u8).sub(0x18) as *mut u32) = ONE;
            *((row as *mut u8).sub(0x14) as *mut u32) = 0;
            *((row as *mut u8).sub(0x10) as *mut u32) = 0;
            *((row as *mut u8).sub(0x8) as *mut u32) = 0;
            *((row as *mut u8).sub(0x4) as *mut u32) = ONE;
            *(row as *mut u32) = 0;
            *((row as *mut u8).add(0x8) as *mut u32) = 0;
            *((row as *mut u8).add(0xC) as *mut u32) = 0;
            *((row as *mut u8).add(0x10) as *mut u32) = ONE;
            *((row as *mut u8).add(0x20) as *mut u32) = 0;
            *((row as *mut u8).add(0x1C) as *mut u32) = 0;
            *((row as *mut u8).add(0x18) as *mut u32) = 0;
            *((row as *mut u8).add(0x9E8) as *mut u32) = ONE;
            *((row as *mut u8).add(0x9EC) as *mut u32) = 0;
            *((row as *mut u8).add(0x9F0) as *mut u32) = 0;
            *((row as *mut u8).add(0x9F4) as *mut u32) = 0;
            *((row as *mut u8).add(0x9F8) as *mut u32) = 0;
            *((row as *mut u8).add(0x9FC) as *mut u32) = ONE;
            *((row as *mut u8).add(0xA00) as *mut u32) = 0;
            *((row as *mut u8).add(0xA04) as *mut u32) = 0;
            *((row as *mut u8).add(0xA08) as *mut u32) = 0;
            *((row as *mut u8).add(0xA0C) as *mut u32) = 0;
            *((row as *mut u8).add(0xA10) as *mut u32) = ONE;
            *((row as *mut u8).add(0xA14) as *mut u32) = 0;
            *((row as *mut u8).add(0xA18) as *mut u32) = 0;
            *((row as *mut u8).add(0xA1C) as *mut u32) = 0;
            *((row as *mut u8).add(0xA20) as *mut u32) = 0;
            *((row as *mut u8).add(0xA24) as *mut u32) = ONE;
            *((row as *mut u8).add(0x4E8) as *mut u32) = ONE;
            *((row as *mut u8).add(0x4EC) as *mut u32) = 0;
            *((row as *mut u8).add(0x4F0) as *mut u32) = 0;
            *((row as *mut u8).add(0x4F8) as *mut u32) = 0;
            *((row as *mut u8).add(0x4FC) as *mut u32) = ONE;
            *((row as *mut u8).add(0x500) as *mut u32) = 0;
            *((row as *mut u8).add(0x508) as *mut u32) = 0;
            *((row as *mut u8).add(0x50C) as *mut u32) = 0;
            *((row as *mut u8).add(0x510) as *mut u32) = ONE;
            *((row as *mut u8).add(0x520) as *mut u32) = 0;
            *((row as *mut u8).add(0x51C) as *mut u32) = 0;
            *((row as *mut u8).add(0x518) as *mut u32) = 0;
            *((row as *mut u8).add(0xEE8) as *mut u32) = ONE;
            *((row as *mut u8).add(0xEEC) as *mut u32) = 0;
            *((row as *mut u8).add(0xEF0) as *mut u32) = 0;
            *((row as *mut u8).add(0xEF4) as *mut u32) = 0;
            *((row as *mut u8).add(0xEF8) as *mut u32) = 0;
            *((row as *mut u8).add(0xEFC) as *mut u32) = ONE;
            *((row as *mut u8).add(0xF00) as *mut u32) = 0;
            *((row as *mut u8).add(0xF04) as *mut u32) = 0;
            *((row as *mut u8).add(0xF08) as *mut u32) = 0;
            *((row as *mut u8).add(0xF0C) as *mut u32) = 0;
            *((row as *mut u8).add(0xF10) as *mut u32) = ONE;
            *((row as *mut u8).add(0xF14) as *mut u32) = 0;
            *((row as *mut u8).add(0xF18) as *mut u32) = 0;
            *((row as *mut u8).add(0xF1C) as *mut u32) = 0;
            *((row as *mut u8).add(0xF20) as *mut u32) = 0;
            *((row as *mut u8).add(0xF24) as *mut u32) = ONE;
            *((s as *mut u8).sub(0x50) as *mut u32) = 0;
            *((byte_cur as *mut u8).add(inner as usize)) = 0;
            *(s as *mut u32) = 0;
            *((blk as *mut u8).add(0x4) as *mut u32) = 0;
            *(blk as *mut u32) = 0;
            *((blk as *mut u8).sub(0x4) as *mut u32) = 0;
            *((col as *mut u8).add(0x4) as *mut u32) = 0;
            *(col as *mut u32) = 0;
            *((col as *mut u8).sub(0x4) as *mut u32) = 0;
                row = row.wrapping_add(0x40);
                col = col.wrapping_add(0x10);
                s = s.wrapping_add(4);
            }
            blk = blk.wrapping_add(0x10);
            byte_cur = byte_cur.wrapping_add(4);
        }
        let f0 = *global::<u32>(VOICE_COPY_0);
        *((obj as *mut u8).add(0x1714) as *mut u32) = f0;
    *((obj as *mut u8).add(0x1400) as *mut u32) = ONE;
    *((obj as *mut u8).add(0x1404) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1408) as *mut u32) = 0;
    *((obj as *mut u8).add(0x140C) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1410) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1414) as *mut u32) = ONE;
    *((obj as *mut u8).add(0x1418) as *mut u32) = 0;
    *((obj as *mut u8).add(0x141C) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1420) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1424) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1428) as *mut u32) = ONE;
    *((obj as *mut u8).add(0x142C) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1430) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1434) as *mut u32) = 0;
    *((obj as *mut u8).add(0x1438) as *mut u32) = 0;
    *((obj as *mut u8).add(0x143C) as *mut u32) = ONE;
        let fa = f32::from_bits(*global::<u32>(VOICE_REF_A));
        let g0 = f32::from_bits(*global::<u32>(VOICE_CMP_0));
        let g1 = f32::from_bits(*global::<u32>(VOICE_CMP_1));
        let g2 = f32::from_bits(*global::<u32>(VOICE_CMP_2));
        let g3 = f32::from_bits(*global::<u32>(VOICE_CMP_3));
        let g4 = f32::from_bits(*global::<u32>(VOICE_CMP_4));
        let g5 = f32::from_bits(*global::<u32>(VOICE_CMP_5));
        // Each probe is an equality check (the flag-parity jump is taken
        // exactly when the compared pair differs, NaN included).
        if g0 == 0.0 {
            if g1 != 0.0 || g2 != fa || g3 != 0.0 || g4 != fa || g5 != 0.0 {
                let fb = f32::from_bits(*global::<u32>(VOICE_REF_B));
                if g1 == fa && g2 == 0.0 && g3 == 0.0 && g4 == 0.0 && g5 == fb {
                    *((obj as *mut u8).add(0x1428) as *mut u32) = NEG_ONE;
                }
            } else {
                *((obj as *mut u8).add(0x1414) as *mut u32) = 0;
                *((obj as *mut u8).add(0x1418) as *mut u32) = ONE;
                *((obj as *mut u8).add(0x1424) as *mut u32) = ONE;
                *((obj as *mut u8).add(0x1428) as *mut u32) = 0;
            }
        }
        *((obj as *mut u8).add(0x1704) as *mut u32) = 1;
        *((obj as *mut u8).add(0x1708) as *mut u32) = 0;
        let idx = *global::<u32>(TLS_QUEUE_INDEX);
        let t = tls_slot(idx as usize);
        *((t as *mut u8).add(0x70) as *mut u32) = 1;
        t
    }
});
