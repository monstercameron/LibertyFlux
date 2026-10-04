// original: 0x00b55490 fetch_scaled_vector_to_fields
/// Fetch a scaled triple through two helpers and store four floats at +0xe0.
///
/// Three shared weights are combined with 1.0/divisor when the source object
/// exists, helper 1 resolves and helper 2 (mode 5) reports ready: the stored
/// triple is then (k*w0, k*w1, k*w2) with k = 1.0/divisor. On the early paths
/// the weights are stored unscaled. The first stored word on the helper paths
/// is the bit pattern 4 (a leftover argument slot the original re-reads as a
/// float through its shifted frame, i.e. the denormal 5.6e-45), and the
/// fourth word is the third weight, except when the source slot is empty,
/// where it is +0.0 from untouched fill. Returns the output pointer.
export!(thiscall, rw_00b55490(this: u32, out: u32, divisor: f32) -> u32 {
    unsafe {
        const W0: u32 = 0x01b4b2a0;
        const W1: u32 = 0x01b4b2a4;
        const W2: u32 = 0x01b4b2a8;
        const ONE: u32 = 0x00fe88e8;
        let g1 = (global::<f32>(W0)).read();
        let g2 = (global::<f32>(W1)).read();
        let g3 = (global::<f32>(W2)).read();
        let store = |v1: f32, v2: f32, v3: f32, v4: f32| {
            ((out + 0xe0) as *mut f32).write(v1);
            ((out + 0xe4) as *mut f32).write(v2);
            ((out + 0xe8) as *mut f32).write(v3);
            ((out + 0xec) as *mut f32).write(v4);
        };
        if ((this + 0x1a18) as *const u32).read() == 0 {
            store(g1, g2, g3, 0.0);
            return out;
        }
        let source: u32 = callee_cdecl!(1, u32, 4);
        // Leftover slot value, re-read as a float (denormal 5.6e-45).
        let d = f32::from_bits(4);
        if source == 0 {
            store(d, g1, g2, g3);
            return out;
        }
        // Scratch block offered to the readiness helper: the leftover slot
        // followed by the three weights the original had just stored there.
        let mut probe = [0u32; 4];
        probe[0] = 4;
        probe[1] = g1.to_bits();
        probe[2] = g2.to_bits();
        probe[3] = g3.to_bits();
        let ready: u32 = callee_thiscall!(
            2,
            u32,
            ((source + 0x10) as *const u32).read(),
            5,
            0,
            probe.as_mut_ptr() as u32
        );
        if ready & 0xff == 0 {
            store(d, g1, g2, g3);
            return out;
        }
        let one = (global::<f32>(ONE)).read();
        let k = one / divisor;
        store(k * d, k * g1, k * g2, g3);
        out
    }
});
