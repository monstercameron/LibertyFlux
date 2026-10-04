// original: 0x0094cff0 angle_range_gate
/// Decide whether a target point falls inside a gated angular range.
///
/// Takes a two-float sample block plus five scalar parameters, derives an
/// angle through a scripted four-argument arctangent helper, wraps it into
/// [0, 2*pi], and passes it through scripted sine and cosine helpers. The
/// resulting direction and offset vectors are run through a scripted
/// three-float vector transform (four calls sharing two output buffers),
/// and the two dot products of the transformed pairs drive four ordered
/// gates: the first dot must be non-negative, the sample spread must cover
/// it, the second dot must be non-negative, and the scaled length must
/// cover the second dot. Returns 1 only when every gate passes, else 0.
/// Any NaN entering a gate comparison fails that gate; angle wrapping
/// skips NaN without looping.
export!(cdecl, rw_0094cff0(obj: u32, a0: f32, a1: f32, a2: f32, a3: f32, a4: f32) -> u32 {
    unsafe {
        const ATAN_ID: u32 = 1;
        const SIN_ID: u32 = 2;
        const COS_ID: u32 = 3;
        const T1_ID: u32 = 4;
        const T2_ID: u32 = 5;
        const T3_ID: u32 = 6;
        const T4_ID: u32 = 7;
        const HALF_PI_GLOB: u32 = 0xFE8978;
        const TWO_PI_GLOB: u32 = 0xFE8AEC;
        const ONE_GLOB: u32 = 0xFE88E8;

        let atan: f32 = callee_cdecl!(
            ATAN_ID,
            f32,
            a0.to_bits(),
            a1.to_bits(),
            a2.to_bits(),
            a3.to_bits()
        );
        let half_pi = *(relocated(HALF_PI_GLOB) as *const f32);
        let two_pi = *(relocated(TWO_PI_GLOB) as *const f32);
        let one = *(relocated(ONE_GLOB) as *const f32);
        let mut ang = atan + half_pi;
        while 0.0 > ang {
            ang += two_pi;
        }
        while ang > two_pi {
            ang -= two_pi;
        }
        let d14 = a2 - a0;
        let d18 = a3 - a1;
        let sinv = f32::from_bits(callee_cdecl!(SIN_ID, u32, ang.to_bits()));
        // The original adds and subtracts the parameter back; the rounding
        // is part of the result, so it is kept, not simplified away.
        let v14 = (sinv * a4 + a0) - a0;
        let v18 = v14;
        let cosv = f32::from_bits(callee_cdecl!(COS_ID, u32, ang.to_bits()));
        // Bitwise negation through the same mask global the original xors
        // with. This must stay a genuine xor: a plain unary minus, or any
        // form the compiler recognises as negation, is folded into a
        // subtraction that keeps +NaN instead of flipping it to -NaN.
        // Loading the mask from the global keeps the xor opaque.
        let mask = *(relocated(0xFE8FA0) as *const u32);
        let neg_cos = f32::from_bits((cosv * a4).to_bits() ^ mask);
        let v1c = (neg_cos + a1) - a1;
        // Order matches the original: d18^2 first, then d14^2.
        let d4 = d18 * d18 + d14 * d14;
        let sqrt4 = d4.sqrt();
        let s14 = (v14 * v14 + v1c * v1c).sqrt();
        let w0 = *(obj as *const f32);
        let w1 = *((obj.wrapping_add(4)) as *const f32);
        let n1 = if d4 == 0.0 { 0.0 } else { one / d4.sqrt() };
        // Third input word of the first transform is a never-written frame
        // slot, i.e. the defined stack fill (zero in the contract).
        let t1in = [(n1 * d14).to_bits(), (n1 * d18).to_bits(), 0u32];
        let mut t1out = [0u32; 3];
        // Later inputs reuse earlier buffers: third words are stale values
        // from previous inputs, reproduced here explicitly.
        let t2in = [(w0 - a0).to_bits(), (w1 - a1).to_bits(), t1in[0]];
        let mut t2out = [0u32; 3];
        let _: u32 = callee_thiscall!(
            T1_ID,
            u32,
            t1out.as_mut_ptr() as u32,
            t1in.as_ptr() as u32,
            1
        );
        let _: u32 = callee_thiscall!(
            T2_ID,
            u32,
            t2out.as_mut_ptr() as u32,
            t2in.as_ptr() as u32,
            1
        );
        let f = |w: u32| f32::from_bits(w);
        // Order matches the original: (y0*x0 + y1*x1) + y2*x2.
        let dot1 = f(t2out[0]) * f(t1out[0]) + f(t2out[1]) * f(t1out[1]);
        let dot1 = dot1 + f(t2out[2]) * f(t1out[2]);
        // Original gates are comiss+jb: fail when less or unordered.
        if !(dot1 >= 0.0) {
            return 0;
        }
        if !(sqrt4 >= dot1) {
            return 0;
        }
        let d2b = v18 * v18 + v1c * v1c;
        let n2 = if d2b == 0.0 { 0.0 } else { one / d2b.sqrt() };
        let t3in = [(v18 * n2).to_bits(), (v1c * n2).to_bits(), t2in[0]];
        let mut t3out = [0u32; 3];
        let t4in = [t2in[0], t2in[1], t1in[0]];
        let mut t4out = [0u32; 3];
        let _: u32 = callee_thiscall!(
            T3_ID,
            u32,
            t3out.as_mut_ptr() as u32,
            t3in.as_ptr() as u32,
            1
        );
        let _: u32 = callee_thiscall!(
            T4_ID,
            u32,
            t4out.as_mut_ptr() as u32,
            t4in.as_ptr() as u32,
            1
        );
        let dot2 = f(t4out[0]) * f(t3out[0]) + f(t4out[1]) * f(t3out[1]);
        let dot2 = dot2 + f(t4out[2]) * f(t3out[2]);
        if !(dot2 >= 0.0) {
            return 0;
        }
        if s14 >= dot2 { 1 } else { 0 }
    }
});
