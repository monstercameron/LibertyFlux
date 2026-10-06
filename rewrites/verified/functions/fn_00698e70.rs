// original: 0x00698e70 unknown (quantized channel sample evaluator)
/// Evaluates a quantized float channel at an integer index: the high bits
/// select a coarse segment whose stored value is dequantized (unsigned int
/// via a signed convert plus the 2^32 table fixup) and scaled by the gain
/// and bias at +0x30/+0x34; the low 5 bits select that many detail terms
/// whose values are summed and scaled by the gain at +0x2c. Returns the
/// total as an f32 in XMM0.
lf_k2_rt::export!(thiscall, rw_00698e70(this: *mut u8, index: u32) -> f32 {
    unsafe {
        let coarse = index >> 5;
        let detail = index & 0x1f;
        let q0: u32 = lf_k2_rt::callee_thiscall!(
            1,
            u32,
            (this as u32).wrapping_add(8),
            coarse
        );
        let fixups = lf_k2_rt::global::<f64>(0x00FE8F50);
        let as_signed = (q0 as i32) as f64 + *fixups.add((q0 >> 31) as usize);
        let gain = f32::from_bits(*((this.add(0x30)) as *const u32));
        let bias = f32::from_bits(*((this.add(0x34)) as *const u32));
        let base = (as_signed as f32) * gain + bias;
        let mut slot = 0u32;
        if coarse != 0 {
            // Answer feeds the handed frame slot (one-word snapshot).
            slot = lf_k2_rt::callee_thiscall!(
                1,
                u32,
                (this as u32).wrapping_add(0x14),
                coarse - 1
            );
        }
        let mut acc = 0u32;
        for _ in 0..detail {
            let term: u32 = lf_k2_rt::callee_thiscall!(
                2,
                u32,
                (this as u32).wrapping_add(0x20),
                (&mut slot as *mut u32) as u32
            );
            acc = acc.wrapping_add(term);
        }
        let dgain = f32::from_bits(*((this.add(0x2c)) as *const u32));
        (acc as i32) as f32 * dgain + base
    }
});
