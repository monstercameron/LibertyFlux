// original: 0x009AB2E0 audio_entry_scaled_add (proposed)

/// Audio entry scaled accumulate: adds a callee-derived value, scaled by a
/// table entry, into an accumulator cell.
///
/// Guards (any failure returns quietly): `idx` must be non-negative, the
/// table pointer at `this + 0x9d0` non-null, `idx` below the 16-bit count at
/// table `+ 0x0a`, and the entry dword at table `+ idx * 25 + 0x10`
/// non-negative. Otherwise the entry is converted to float, multiplied by
/// the constant 1.25 (`SCALE`), truncated back to int (x86 `cvttss2si`
/// semantics: NaN and out-of-range give `i32::MIN`, not saturation), passed
/// with the entry to the combine callee, and the accumulator cell at table
/// `+ idx * 25 + 0x21` becomes `add + callee_result`. The table pointer is
/// re-read after the call. Float operation order matches the original.
/// Original: 0x009AB2E0 (thiscall, two stack words). Returns nothing.
lf_checker_rt::export!(thiscall, rw_009AB2E0(this: u32, idx: u32, add: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x9d0;
        const COUNT_OFF: u32 = 0x0a;
        const ROW: u32 = 25;
        const ENTRY_OFF: u32 = 0x10;
        const ACCUM_OFF: u32 = 0x21;
        const SCALE: u32 = 0x00fe8920;
        const COMBINE: u32 = 1;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate exactly like x86 `cvttss2si` (differs from Rust `as`).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x > -2147483648.0 && x < 2147483648.0 {
                x as i32
            } else {
                i32::MIN
            }
        }

        if (idx as i32) < 0 {
            return 0;
        }
        let table = ((this.wrapping_add(TABLE_PTR)) as *const u32).read_unaligned();
        if table == 0 {
            return 0;
        }
        if (idx as i32) <= -1 {
            return 0;
        }
        let count = ((table.wrapping_add(COUNT_OFF)) as *const u16).read_unaligned() as u32;
        if idx >= count {
            return 0;
        }
        let row = table.wrapping_add(idx.wrapping_mul(ROW));
        let entry = ((row.wrapping_add(ENTRY_OFF)) as *const u32).read_unaligned();
        if (entry as i32) <= -1 {
            return 0;
        }
        let scale = f32::from_bits(
            (lf_checker_rt::global::<u32>(SCALE) as *const u32).read_unaligned(),
        );
        let n = cvtt(mul(entry as i32 as f32, scale));
        let r: u32 = lf_checker_rt::callee_cdecl!(COMBINE, u32, entry, n as u32);
        let table2 = ((this.wrapping_add(TABLE_PTR)) as *const u32).read_unaligned();
        let cell = table2.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(ACCUM_OFF);
        ((cell) as *mut u32).write_unaligned(add.wrapping_add(r));
        0
    }
});
