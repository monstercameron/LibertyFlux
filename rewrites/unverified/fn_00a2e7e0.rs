// original: 0x00A2E7E0 files_mem_tag_search (proposed)

/// Search a 128-entry signed tag table for an expected value, two passes.
///
/// The target starts as the current value, or the alternate value when the
/// reference float is strictly above the current float (ordered comparison:
/// NaN and equality both keep the current value), then advances by one
/// modulo 8 (signed C remainder, so the target lies in -7..7).
///
/// Pass 1 walks a cursor over the table, three probes per round: a probe
/// matches when both the byte at `(cursor + 127) mod 128` and the byte at
/// the cursor equal the target (signed bytes, signed comparison). The
/// cursor is stored back after every step except the first probe. After
/// more than 0x80 steps without a match, pass 2 retries with the probe
/// index biased by 0x7f or 0x80 (again chosen by the float comparison)
/// and a single comparison per probe. On any match, or when pass 2 also
/// exhausts its steps, the byte at `(cursor + 127) mod 128` is stored to
/// the alternate slot, the byte at the cursor to the current slot, and the
/// latter (sign-extended) is returned. All indexes use the signed C
/// remainder, so a negative cursor reads below the table base.
///
/// Original: 0x00A2E7E0 (cdecl, no arguments, plain `ret`).
lf_checker_rt::export!(cdecl, rw_00A2E7E0() -> u32 {
    unsafe {
        const REF_FLOAT: u32 = 0x00FE_8830;
        const CUR_FLOAT: u32 = 0x012D_DE94;
        const CUR_VAL: u32 = 0x012D_DE84;
        const ALT_VAL: u32 = 0x012D_DE80;
        const CURSOR: u32 = 0x012D_DE90;
        const TABLE: u32 = 0x0103_C900;
        const STEP_LIMIT: i32 = 0x80;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(a).write(v) }
        }
        /// Signed table byte at a possibly negative index (the original's
        /// `movsx` from `[index + TABLE]` with 32-bit wrapping).
        #[inline(always)]
        unsafe fn tag(idx: i32) -> i32 {
            unsafe {
                let va = (TABLE as i32).wrapping_add(idx) as u32;
                (lf_checker_rt::global::<i8>(va) as *const i8).read() as i32
            }
        }
        /// The original's lowering (`and 0x8000007f` plus fix-up) is the
        /// signed C remainder; Rust `%` on `i32` is exactly that.
        #[inline(always)]
        fn mod128(x: i32) -> i32 {
            x % 128
        }

        let ref_f = f32::from_bits(rd32(REF_FLOAT));
        let cur_f = f32::from_bits(rd32(CUR_FLOAT));
        // `comiss` + `cmova`: move only when strictly above (ordered).
        let above = ref_f > cur_f;
        let mut target: i32 = rd32(CUR_VAL) as i32;
        if above {
            target = rd32(ALT_VAL) as i32;
        }
        target = target.wrapping_add(1) % 8;

        let mut cursor: i32 = rd32(CURSOR) as i32;
        let mut steps: i32 = 0;
        let mut found = false;
        loop {
            if tag(mod128(cursor.wrapping_add(0x7f))) == target
                && tag(cursor) == target
            {
                found = true;
                break;
            }
            cursor = mod128(cursor.wrapping_add(1));
            wr32(CURSOR, cursor as u32);
            if tag(mod128(cursor.wrapping_add(0x7f))) == target
                && tag(cursor) == target
            {
                steps += 1;
                found = true;
                break;
            }
            cursor = mod128(cursor.wrapping_add(1));
            wr32(CURSOR, cursor as u32);
            if tag(mod128(cursor.wrapping_add(0x7f))) == target
                && tag(cursor) == target
            {
                steps += 2;
                found = true;
                break;
            }
            cursor = mod128(cursor.wrapping_add(1));
            steps += 3;
            wr32(CURSOR, cursor as u32);
            if steps > STEP_LIMIT {
                break;
            }
        }
        if !(found && steps <= STEP_LIMIT) {
            let bias: i32 = if above { -1 } else { 0 };
            steps = 0;
            loop {
                let at = mod128(bias.wrapping_add(0x80).wrapping_add(cursor));
                if tag(at) == target {
                    break;
                }
                cursor = mod128(cursor.wrapping_add(1));
                wr32(CURSOR, cursor as u32);
                let at = mod128(bias.wrapping_add(0x80).wrapping_add(cursor));
                if tag(at) == target {
                    break;
                }
                cursor = mod128(cursor.wrapping_add(1));
                wr32(CURSOR, cursor as u32);
                let at = mod128(bias.wrapping_add(0x80).wrapping_add(cursor));
                if tag(at) == target {
                    break;
                }
                cursor = mod128(cursor.wrapping_add(1));
                steps += 3;
                wr32(CURSOR, cursor as u32);
                if steps > STEP_LIMIT {
                    break;
                }
            }
        }
        wr32(ALT_VAL, tag(mod128(cursor.wrapping_add(0x7f))) as u32);
        let out = tag(cursor);
        wr32(CUR_VAL, out as u32);
        out as u32
    }
});
