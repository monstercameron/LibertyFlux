// original: 0x00BE5090 facing_dot_classify (proposed)

/// Classify a ped's seating approach from its facing dot-product.
///
/// `this` points to the task object (pointer at `+0x18` to a record whose
/// `+0x20` is a reference matrix); `ped` points to the ped (vtable at `+0x0`,
/// matrix at `+0x20`, state object at `+0x2c4`, mode word at `+0xb80`).
/// Computes the dot product of the ped matrix's second row (`+0x0`, `+0x4`,
/// `+0x8`) with the displacement between the ped and reference positions
/// (`+0x30`, `+0x34`, `+0x38`), in the original's operation order.
///
/// If the state object exists and the vtable slot at `+0x12c` (called with
/// the ped) returns the sign-extended word at state `+0x2e`, the result
/// depends only on the sign of the dot product: `0x99` when it is at least
/// `+0.0`, `-1` otherwise (an unordered NaN comparison also gives `-1`).
///
/// Otherwise the dot product, converted exactly to f64, is compared against
/// `-0.2` and `+0.2` (constants read from the image): below `-0.2` gives
/// `0x98` (`0x9a` when the mode word is 3 or 4), strictly between gives
/// `0x96` (an NaN dot product also gives `0x96`, matching the unordered
/// comisd), at least `+0.2` gives `0x97` (`0x99` when the mode is 3 or 4).
/// A null task record returns `-1` without further reads.
///
/// Original: 0x00BE5090 (thiscall, one stack word, eax return).
lf_checker_rt::export!(thiscall, rw_00BE5090(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_RECORD: u32 = 0x18;
        const RECORD_MATRIX: u32 = 0x20;
        const PED_MATRIX: u32 = 0x20;
        const PED_STATE: u32 = 0x2c4;
        const PED_MODE: u32 = 0xb80;
        const STATE_WANT: u32 = 0x2e;
        const VT_SLOT: u32 = 0x12c;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const ROW_X: u32 = 0x0;
        const ROW_Y: u32 = 0x4;
        const ROW_Z: u32 = 0x8;
        const ZERO_F: u32 = 0x00FE8628;
        const NEG_PT2: u32 = 0x00EB9508;
        const POS_PT2: u32 = 0x00EA87B8;
        const RET_NONE: u32 = 0xFFFF_FFFF;
        const RET_SIGN_POS: u32 = 0x99;
        const RET_MID: u32 = 0x96;
        const RET_LO_BASE: u32 = 0x98;
        const RET_HI_BASE: u32 = 0x97;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let mode = rd32(ped + PED_MODE);
        let flag = mode == 3 || mode == 4;
        let rec = rd32(this + TASK_RECORD);
        if rec == 0 {
            return RET_NONE;
        }
        let pm = rd32(ped + PED_MATRIX);
        let rm = rd32(rec + RECORD_MATRIX);
        let dx = sub(rdf(pm + POS_Y), rdf(rm + POS_Y));
        let dy = sub(rdf(pm + POS_X), rdf(rm + POS_X));
        let dz = sub(rdf(pm + POS_Z), rdf(rm + POS_Z));
        let t1 = mul(rdf(pm + ROW_Y), dx);
        let t2 = mul(rdf(pm + ROW_X), dy);
        let t3 = add(t1, t2);
        let t4 = mul(rdf(pm + ROW_Z), dz);
        let dot = add(t3, t4);

        let state = rd32(ped + PED_STATE);
        if state != 0 {
            let want = rd16(state + STATE_WANT) as i16 as u32;
            let target: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(ped) + VT_SLOT) as usize);
            if target(ped) == want {
                let zero = f32::from_bits(rd32(lf_checker_rt::relocated(ZERO_F)));
                if dot >= zero {
                    return RET_SIGN_POS;
                }
                return RET_NONE;
            }
        }

        let lo = f64::from_bits(
            (rd32(lf_checker_rt::relocated(NEG_PT2)) as u64)
                | ((rd32(lf_checker_rt::relocated(NEG_PT2) + 4) as u64) << 32),
        );
        let hi = f64::from_bits(
            (rd32(lf_checker_rt::relocated(POS_PT2)) as u64)
                | ((rd32(lf_checker_rt::relocated(POS_PT2) + 4) as u64) << 32),
        );
        let d = dot as f64;
        // NaN falls through to the second check (jbe is taken when unordered).
        if d < lo {
            return RET_LO_BASE + (flag as u32) * 2;
        }
        // NaN takes the jb as well, hence the explicit unordered arm.
        if d < hi || d.is_nan() {
            return RET_MID;
        }
        RET_HI_BASE + (flag as u32) * 2
    }
});
