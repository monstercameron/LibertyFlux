// original: 0x00be4f60 ped_react_code (proposed)

/// Pick a reaction code (0x91-0x96) for a ped facing a task object.
///
/// `task` points to the task; its inner object at `+0x14` holds a matrix
/// pointer at `+0x20` (position at `+0x30` past it) or, when null, an inline
/// position at `+0x10`. `ped` points to the ped; its matrix at `+0x20` holds
/// a direction row at `+0x0`/`+0x4`/`+0x8` and a position at
/// `+0x30`/`+0x34`/`+0x38`.
///
/// The function dots the ped's direction row with (ped position minus object
/// position), all in the original's operand order. It then asks the ped-state
/// callee (thiscall on `ped + 0x2b0` with argument 1): a non-zero low byte
/// returns the near pair. Otherwise, when the ped's flag byte at `+0x219` is
/// set and the counter at `[[ped + 0x228] + 0x568]` is above the global
/// threshold (unsigned), it returns the far pair. Otherwise the dot product
/// is classified against three float constants and one double constant from
/// the image: inside (-1.2, -0.2) returns the near pair, inside (0.2, 1.2)
/// returns the mid pair, anything else (including NaN) returns the far pair.
/// The high or low member of the pair is picked by the mode word at
/// `ped + 0xb80` being 3 or 4.
///
/// Near pair 0x91/0x94, mid pair 0x92/0x95, far pair 0x93/0x96; the second of
/// each pair is used when the mode word is 3 or 4.
///
/// The original also writes its scratch below its frame (unobserved) and the
/// mode flag into the top byte of its incoming argument slot; that slot is
/// dead after the callee-popped return, so the rewrite omits both writes.
///
/// Original: 0x00be4f60 (thiscall, ecx = task, one stack word = ped).
lf_checker_rt::export!(thiscall, rw_00be4f60(task: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_INNER: u32 = 0x14;
        const INNER_MATRIX: u32 = 0x20;
        const MATRIX_POS: u32 = 0x30;
        const INLINE_POS: u32 = 0x10;
        const PED_MATRIX: u32 = 0x20;
        const PED_STATE: u32 = 0x2b0;
        const PED_MODE: u32 = 0xb80;
        const PED_FLAG: u32 = 0x219;
        const PED_COUNTER_PTR: u32 = 0x228;
        const COUNTER_VALUE: u32 = 0x568;
        const THRESHOLD: u32 = 0x11735b4;
        const C_LOWER: u32 = 0xfe8d68; // -0.2f
        const C_FAR: u32 = 0xeb9504; // -1.2f
        const C_NEAR: u32 = 0xfe891c; // +1.2f
        const C_MID: u32 = 0xea87b8; // 0.2 (double)
        const STATE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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

        // Object position: through the matrix, or the inline fallback.
        let inner = rd32(task.wrapping_add(TASK_INNER));
        let mat = rd32(inner.wrapping_add(INNER_MATRIX));
        let pos = if mat != 0 {
            mat.wrapping_add(MATRIX_POS)
        } else {
            inner.wrapping_add(INLINE_POS)
        };
        // Facing dot separation, in the original's operand order.
        let m = rd32(ped.wrapping_add(PED_MATRIX));
        let dy = sub(rdf(m.wrapping_add(0x34)), rdf(pos.wrapping_add(4)));
        let dx = sub(rdf(m.wrapping_add(0x30)), rdf(pos));
        let dz = sub(rdf(m.wrapping_add(0x38)), rdf(pos.wrapping_add(8)));
        let dist = add(
            add(mul(rdf(m.wrapping_add(4)), dy), mul(rdf(m), dx)),
            mul(rdf(m.wrapping_add(8)), dz),
        );
        // Mode flag: the high member of each pair when the mode is 3 or 4.
        let mode = rd32(ped.wrapping_add(PED_MODE));
        let hi = mode == 3 || mode == 4;
        // Ped-state callee; only its low byte is observed.
        let ans: u32 =
            lf_checker_rt::callee_thiscall!(STATE_CALLEE, u32, ped.wrapping_add(PED_STATE), 1u32);
        if ans & 0xff != 0 {
            return if hi { 0x94 } else { 0x91 };
        }
        if rd8(ped.wrapping_add(PED_FLAG)) != 0 {
            let counter = rd32(rd32(ped.wrapping_add(PED_COUNTER_PTR)).wrapping_add(COUNTER_VALUE));
            let threshold = rd32(lf_checker_rt::relocated(THRESHOLD));
            if counter > threshold {
                return if hi { 0x96 } else { 0x93 };
            }
        }
        let c_lower = rdf(lf_checker_rt::relocated(C_LOWER));
        let c_far = rdf(lf_checker_rt::relocated(C_FAR));
        if c_lower > dist && dist > c_far {
            return if hi { 0x94 } else { 0x91 };
        }
        let c_mid = f64::from_bits(
            (rd32(lf_checker_rt::relocated(C_MID)) as u64)
                | ((rd32(lf_checker_rt::relocated(C_MID).wrapping_add(4)) as u64) << 32),
        );
        let c_near = rdf(lf_checker_rt::relocated(C_NEAR));
        if (dist as f64) > c_mid && c_near > dist {
            return if hi { 0x95 } else { 0x92 };
        }
        if hi {
            0x96
        } else {
            0x93
        }
    }
});
